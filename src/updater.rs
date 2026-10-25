//! Optional updates: reads a small JSON file from GitHub Pages and, on a click, swaps in the new app.
//! Off by default (Settings > Startup). It shells out to macOS's own curl, so no HTTP library is linked.
// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

use serde::Deserialize;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::menu::MenuItem;
use tauri::{AppHandle, Manager, Wry};

const LATEST_URL: &str = "https://kacpergorec.github.io/sidecal/latest.json";
const DOWNLOAD_PREFIX: &str = "https://github.com/kacpergorec/sidecal/releases/download/";
pub const CHECK_TITLE: &str = "Check for Updates…";

#[derive(Deserialize, Clone)]
struct Asset {
    url: String,
    sha256: String,
}

#[derive(Deserialize)]
struct Latest {
    version: String,
    #[serde(default)]
    assets: HashMap<String, Asset>,
}

#[derive(Clone)]
struct Found {
    version: String,
    asset: Asset,
}

pub struct Updater {
    item: MenuItem<Wry>,
    found: Mutex<Option<Found>>,
    busy: Mutex<bool>,
}

pub fn init(app: &AppHandle, item: MenuItem<Wry>) {
    app.manage(Updater { item, found: Mutex::new(None), busy: Mutex::new(false) });
}

fn parse_version(v: &str) -> Option<(u64, u64, u64)> {
    let mut p = v.trim().trim_start_matches('v').split('.').map(|x| x.parse::<u64>().ok());
    Some((p.next()??, p.next()??, p.next()??))
}

fn curl(args: &[&str]) -> Result<Vec<u8>, String> {
    let out = Command::new("/usr/bin/curl")
        .args(["-fsSL", "--max-time", "300"])
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() { Ok(out.stdout) } else { Err("Download failed".into()) }
}

/// Fetches latest.json; `Some` only when it names a newer version with a download for this Mac.
fn check() -> Result<Option<Found>, String> {
    let latest: Latest = serde_json::from_slice(&curl(&[LATEST_URL])?).map_err(|e| e.to_string())?;
    let (Some(new), Some(cur)) = (parse_version(&latest.version), parse_version(env!("CARGO_PKG_VERSION"))) else {
        return Err("Unreadable version".into());
    };
    if new <= cur {
        return Ok(None);
    }
    let key = if cfg!(target_arch = "aarch64") { "aarch64" } else { "x86_64" };
    let asset = latest.assets.get(key).cloned().ok_or("No download for this Mac")?;
    if !asset.url.starts_with(DOWNLOAD_PREFIX) {
        return Err("Unexpected download address".into());
    }
    Ok(Some(Found { version: latest.version, asset }))
}

fn set_title(app: &AppHandle, text: &str) {
    if let Some(u) = app.try_state::<Updater>() {
        let _ = u.item.set_text(text);
    }
}

fn resting_title(app: &AppHandle) -> String {
    match app.state::<Updater>().found.lock().unwrap().as_ref() {
        Some(f) => format!("Install SideCal {}", f.version),
        None => CHECK_TITLE.into(),
    }
}

/// Menu click: first click checks, and once an update is known the next click installs it.
pub fn on_click(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        let u = app.state::<Updater>();
        {
            let mut busy = u.busy.lock().unwrap();
            if *busy {
                return;
            }
            *busy = true;
        }
        let known = u.found.lock().unwrap().clone();
        let result = match known {
            Some(f) => {
                set_title(&app, "Installing update…");
                install(&app, &f)
            }
            None => {
                set_title(&app, "Checking…");
                match check() {
                    Ok(Some(f)) => {
                        *u.found.lock().unwrap() = Some(f);
                        Ok(())
                    }
                    Ok(None) => {
                        set_title(&app, "SideCal is up to date");
                        std::thread::sleep(Duration::from_secs(3));
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
        };
        if let Err(e) = result {
            set_title(&app, &format!("Update failed: {e}"));
            std::thread::sleep(Duration::from_secs(5));
        }
        set_title(&app, &resting_title(&app));
        *u.busy.lock().unwrap() = false;
    });
}

/// Once a day, only while "Check for Updates Automatically" is on. A found update shows in the tray menu.
pub fn spawn_auto_check(app: AppHandle) {
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(15));
        let mut last: Option<Instant> = None;
        loop {
            let enabled = app.state::<crate::Runtime>().settings.lock().unwrap().check_for_updates;
            let due = last.map_or(true, |t| t.elapsed() >= Duration::from_secs(24 * 3600));
            if enabled && due {
                last = Some(Instant::now());
                if let Ok(Some(f)) = check() {
                    *app.state::<Updater>().found.lock().unwrap() = Some(f);
                    set_title(&app, &resting_title(&app));
                }
            }
            std::thread::sleep(Duration::from_secs(1800));
        }
    });
}

fn install(app: &AppHandle, f: &Found) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    // .../SideCal.app/Contents/MacOS/sidecal
    let bundle = exe
        .ancestors()
        .nth(3)
        .filter(|p| p.extension().map_or(false, |e| e == "app"))
        .ok_or("Not running from SideCal.app")?
        .to_path_buf();
    let parent = bundle.parent().ok_or("Bad app location")?;
    // Fail now, while the app is still running, if the folder cannot be written.
    let probe = parent.join(".sidecal-write-test");
    std::fs::File::create(&probe).map_err(|_| "Cannot write to the app's folder")?;
    let _ = std::fs::remove_file(&probe);

    let dir = std::env::temp_dir().join(format!("sidecal-update-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let zip = dir.join("SideCal.zip");
    let zip_arg = zip.to_string_lossy().into_owned();
    curl(&["-o", zip_arg.as_str(), f.asset.url.as_str()])?;

    let sum = Command::new("/usr/bin/shasum").args(["-a", "256"]).arg(&zip).output().map_err(|e| e.to_string())?;
    let sum = String::from_utf8_lossy(&sum.stdout);
    if !sum.split_whitespace().next().unwrap_or("").eq_ignore_ascii_case(&f.asset.sha256) {
        let _ = std::fs::remove_dir_all(&dir);
        return Err("Checksum mismatch".into());
    }
    let unzip = Command::new("/usr/bin/ditto").args(["-x", "-k"]).arg(&zip).arg(&dir).status().map_err(|e| e.to_string())?;
    let fresh = dir.join("SideCal.app");
    if !unzip.success() || !fresh.is_dir() {
        return Err("Could not unpack the update".into());
    }

    let quote = |p: &std::path::Path| -> Result<String, String> {
        let s = p.to_string_lossy();
        if s.contains('\'') { Err("Unsupported characters in the app path".into()) } else { Ok(format!("'{s}'")) }
    };
    // Runs after SideCal quits: swap the bundle (restoring the old one if that fails) and start it again.
    let script = format!(
        "#!/bin/sh\nwhile kill -0 {pid} 2>/dev/null; do sleep 0.2; done\nAPP={app}\nNEW={new}\nBAK=\"$APP.old\"\nrm -rf \"$BAK\"\n\
         if mv \"$APP\" \"$BAK\" && mv \"$NEW\" \"$APP\"; then rm -rf \"$BAK\"; else rm -rf \"$APP\"; mv \"$BAK\" \"$APP\"; fi\n\
         open \"$APP\"\nrm -rf {dir}\n",
        pid = std::process::id(),
        app = quote(&bundle)?,
        new = quote(&fresh)?,
        dir = quote(&dir)?,
    );
    let path = dir.join("update.sh");
    std::fs::write(&path, script).map_err(|e| e.to_string())?;
    Command::new("/bin/sh")
        .arg(&path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    app.exit(0);
    Ok(())
}
