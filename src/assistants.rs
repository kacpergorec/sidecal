//! "Add to Claude and Cursor": registers `sidecal --mcp` with the assistants installed on this Mac.
//! It only edits their config files and runs `claude mcp add`; nothing touches the network.
// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Sets `mcpServers.sidecal` in a JSON config and keeps everything else. A file that is not valid JSON is left alone;
/// an existing file is copied to `.bak` first.
fn add_to_json(path: &Path, exe: &str) -> Result<(), String> {
    let mut root: Value = match std::fs::read_to_string(path) {
        Ok(s) if s.trim().is_empty() => json!({}),
        Ok(s) => serde_json::from_str(&s).map_err(|_| "its config is not valid JSON, left untouched".to_string())?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => json!({}),
        Err(e) => return Err(e.to_string()),
    };
    let servers = root
        .as_object_mut()
        .ok_or("its config is not a JSON object")?
        .entry("mcpServers")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .ok_or("mcpServers in its config is not an object")?;
    servers.insert("sidecal".into(), json!({ "command": exe, "args": ["--mcp"] }));
    let text = serde_json::to_string_pretty(&root).map_err(|e| e.to_string())?;
    if path.exists() {
        let _ = std::fs::copy(path, path.with_extension("json.bak"));
    }
    std::fs::write(path, text + "\n").map_err(|e| e.to_string())
}

/// Registers the server with Claude Code for the current user, replacing an older entry. Runs in a login shell
/// so a `claude` installed through npm, Homebrew or the native installer is found on PATH.
fn add_to_claude_code(exe: &str) -> Result<bool, String> {
    let script = r#"command -v claude >/dev/null 2>&1 || exit 127
claude mcp remove sidecal --scope user >/dev/null 2>&1
claude mcp add --scope user sidecal -- "$0" --mcp"#;
    let out = Command::new("/bin/zsh")
        .args(["-lc", script, exe])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| e.to_string())?;
    match out.status.code() {
        Some(0) => Ok(true),
        Some(127) => Ok(false),
        _ => Err(String::from_utf8_lossy(&out.stderr).trim().to_string()),
    }
}

/// One status line per assistant, shown in Settings.
pub fn connect() -> Vec<String> {
    let exe = match std::env::current_exe() {
        Ok(p) => p.to_string_lossy().into_owned(),
        Err(e) => return vec![format!("Could not find the SideCal app: {e}")],
    };
    let home = match std::env::var_os("HOME") {
        Some(h) => PathBuf::from(h),
        None => return vec!["Could not find your home folder.".into()],
    };
    let mut lines = Vec::new();
    let mut json_target = |name: &str, dir: PathBuf, file: &str, note: &str| {
        lines.push(if !dir.is_dir() {
            format!("{name}: not installed")
        } else {
            match add_to_json(&dir.join(file), &exe) {
                Ok(()) => format!("{name}: added. {note}"),
                Err(e) => format!("{name}: {e}"),
            }
        });
    };
    json_target(
        "Claude Desktop",
        home.join("Library/Application Support/Claude"),
        "claude_desktop_config.json",
        "Restart it to load SideCal.",
    );
    json_target("Cursor", home.join(".cursor"), "mcp.json", "Restart it to load SideCal.");
    lines.push(match add_to_claude_code(&exe) {
        Ok(true) => "Claude Code: added for your user. New sessions can use SideCal.".into(),
        Ok(false) => "Claude Code: not installed".into(),
        Err(e) => format!("Claude Code: {e}"),
    });
    lines
}
