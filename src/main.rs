#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

mod assistants;
mod calendar;
mod i18n;
mod mcp;
mod settings;
mod updater;

use settings::Settings;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::window::Color;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const TRAY_ID: &str = "main";
const MENU_BAR: f64 = 25.0;

struct Runtime {
    settings: Mutex<Settings>,
    /// True while the add/edit form is open: auto-hide must not close it.
    pinned: AtomicBool,
    /// Auto-hide ignores the cursor until this moment (after a manual show).
    grace_until: Mutex<Instant>,
    auto_hidden: AtomicBool,
    /// Bumped per slide; an older slide stops when it changes.
    anim: AtomicU64,
}

fn grace(app: &AppHandle, secs: u64) {
    *app.state::<Runtime>().grace_until.lock().unwrap() = Instant::now() + Duration::from_secs(secs);
}

// ---- commands -------------------------------------------------------------

/// Settings > General > Add to Claude and Cursor. Runs off the main thread: it may start a shell.
#[tauri::command]
async fn connect_assistants() -> Vec<String> {
    tauri::async_runtime::spawn_blocking(assistants::connect).await.unwrap_or_default()
}

#[tauri::command]
fn has_access() -> bool {
    calendar::has_access()
}

#[tauri::command]
fn get_events(start: f64, end: f64) -> Vec<calendar::Event> {
    calendar::events(start, end)
}

#[tauri::command]
fn get_calendars() -> Vec<calendar::Calendar> {
    calendar::calendars()
}

#[tauri::command]
fn save_event(event: calendar::EventInput) -> Result<(), String> {
    calendar::save(event)
}

/// Shows the event in the macOS Calendar app (which also hosts Google/other synced calendars).
#[tauri::command]
fn open_event(id: String) -> Result<(), String> {
    let enc: String = id
        .bytes()
        .map(|b| if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' { (b as char).to_string() } else { format!("%{b:02X}") })
        .collect();
    std::process::Command::new("open")
        .arg(format!("ical://ekevent/{enc}?method=show&options=more"))
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_event(id: String) -> Result<(), String> {
    calendar::remove(&id)
}

#[tauri::command]
fn set_title(app: AppHandle, title: String) {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_title(Some(title));
    }
}

#[tauri::command]
fn set_pinned(state: State<Runtime>, pinned: bool) {
    state.pinned.store(pinned, Ordering::Relaxed);
}

#[tauri::command]
fn get_settings(state: State<Runtime>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn save_settings(app: AppHandle, mut settings: Settings) -> Result<(), String> {
    // Event colours are owned by the sidebar; a stale settings window must not overwrite them.
    {
        let current = app.state::<Runtime>();
        let current = current.settings.lock().unwrap();
        settings.event_colors = current.event_colors.clone();
        settings.hidden_events = current.hidden_events.clone();
    }
    let settings = settings.sanitized();
    settings::save(&app, &settings)?;
    *app.state::<Runtime>().settings.lock().unwrap() = settings;
    apply(&app);
    Ok(())
}

/// Hides or shows one event in the sidebar (it stays in the calendar).
#[tauri::command]
fn set_event_hidden(app: AppHandle, id: String, hidden: bool) -> Result<(), String> {
    let state = app.state::<Runtime>();
    let snapshot = {
        let mut guard = state.settings.lock().unwrap();
        guard.hidden_events.retain(|x| x != &id);
        if hidden {
            guard.hidden_events.push(id);
        }
        guard.clone()
    };
    settings::save(&app, &snapshot)?;
    let _ = app.emit("settings-changed", ());
    Ok(())
}

/// Sets (`Some`) or clears (`None`) the custom colour of one event.
#[tauri::command]
fn set_event_color(app: AppHandle, id: String, color: Option<String>) -> Result<(), String> {
    let state = app.state::<Runtime>();
    let snapshot = {
        let mut guard = state.settings.lock().unwrap();
        match color {
            Some(c) => guard.event_colors.insert(id, c),
            None => guard.event_colors.remove(&id),
        };
        guard.clone()
    };
    settings::save(&app, &snapshot)?;
    let _ = app.emit("settings-changed", ());
    Ok(())
}

#[tauri::command]
fn slide_mode_enabled(app: AppHandle, state: State<Runtime>) -> bool {
    let side = state.settings.lock().unwrap().side.clone();
    slide_mode(&app, &side)
}

#[tauri::command]
fn is_collapsed(state: State<Runtime>) -> bool {
    state.auto_hidden.load(Ordering::Relaxed)
}

#[tauri::command]
fn open_settings(app: AppHandle) {
    show_settings(&app);
}

// ---- windows --------------------------------------------------------------

fn show_settings(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.show();
        let _ = w.set_focus();
        return;
    }
    let theme = app.state::<Runtime>().settings.lock().unwrap().theme.clone();
    let dark = match theme.as_str() {
        "light" => false,
        "dark" => true,
        _ => app
            .get_webview_window("sidebar")
            .and_then(|w| w.theme().ok())
            .map(|t| t == tauri::Theme::Dark)
            .unwrap_or(true),
    };
    let bg = if dark { Color(28, 28, 27, 255) } else { Color(251, 251, 250, 255) };
    let _ = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("settings.html".into()))
        .background_color(bg)
        .title("SideCal Settings")
        .inner_size(760.0, 660.0)
        .min_inner_size(560.0, 480.0)
        .resizable(true)
        .center()
        .build()
        .map(|w| w.set_focus());
}

fn show_sidebar(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("sidebar") {
        if app.state::<Runtime>().auto_hidden.load(Ordering::Relaxed) {
            set_collapsed(app, false);
        }
        grace(app, 2);
        let _ = win.show();
    }
}

fn toggle_sidebar(app: &AppHandle) {
    let rt = app.state::<Runtime>();
    let auto_hide = rt.settings.lock().unwrap().auto_hide;
    let collapsed = rt.auto_hidden.load(Ordering::Relaxed);
    if let Some(win) = app.get_webview_window("sidebar") {
        let visible = win.is_visible().unwrap_or(false);
        if auto_hide && visible {
            if collapsed {
                grace(app, 2);
            }
            set_collapsed(app, !collapsed);
        } else if visible {
            let _ = win.hide();
        } else {
            show_sidebar(app);
        }
    }
}

/// The window keeps its full width and only slides on/off the screen edge (no webview resize,
/// so nothing can lag or jitter). That is only safe when no other monitor sits on the docked
/// side, otherwise the hidden part would show up there; then it is resized instead.
fn slide_mode(app: &AppHandle, side: &str) -> bool {
    let (Ok(Some(main)), Ok(all)) = (app.primary_monitor(), app.available_monitors()) else { return true };
    let (mx, mw) = (main.position().x, main.size().width as i32);
    !all.iter().any(|o| {
        let (ox, ow) = (o.position().x, o.size().width as i32);
        if (ox, ow) == (mx, mw) && o.position().y == main.position().y {
            return false; // the primary monitor itself
        }
        if side == "left" { ox + ow <= mx + 1 } else { ox >= mx + mw - 1 }
    })
}

/// Window x (logical) for the slide mode.
fn slide_x(side: &str, width: f64, strip: f64, screen_w: f64, collapsed: bool) -> f64 {
    match (side == "left", collapsed) {
        (true, false) => 0.0,
        (true, true) => -(width - strip),
        (false, false) => screen_w - width,
        (false, true) => screen_w - strip,
    }
}

/// Docks the window to the configured edge: full width, or a thin strip when collapsed.
fn place(app: &AppHandle, win: &tauri::WebviewWindow, s: &Settings, collapsed: bool) {
    let Ok(Some(m)) = app.primary_monitor() else { return };
    let size = m.size().to_logical::<f64>(m.scale_factor());
    let height = size.height - MENU_BAR;
    if slide_mode(app, &s.side) {
        let w = s.window_width();
        let _ = win.set_size(tauri::LogicalSize::new(w, height));
        let _ = win.set_position(tauri::LogicalPosition::new(slide_x(&s.side, w, s.strip_width, size.width, collapsed), MENU_BAR));
        return;
    }
    let width = if collapsed { s.strip_width } else { s.window_width() };
    let x = if s.side == "left" { 0.0 } else { size.width - width };
    let _ = win.set_size(tauri::LogicalSize::new(width, height));
    let _ = win.set_position(tauri::LogicalPosition::new(x, MENU_BAR));
}

/// Slides the sidebar in/out (ease-out, ~140 ms). Each frame is one non-blocking NSWindow
/// `setFrame`, so the webview keeps painting while the window moves.
fn set_collapsed(app: &AppHandle, collapsed: bool) {
    let rt = app.state::<Runtime>();
    rt.auto_hidden.store(collapsed, Ordering::Relaxed);
    // The UI transitions the same elements while the window slides.
    let _ = app.emit("collapsed", collapsed);
    if !collapsed {
        if let Some(win) = app.get_webview_window("sidebar") {
            let _ = win.set_ignore_cursor_events(false);
            let _ = win.show();
        }
    }
    let gen = rt.anim.fetch_add(1, Ordering::Relaxed) + 1;
    let s = rt.settings.lock().unwrap().clone();
    let app = app.clone();
    std::thread::spawn(move || {
        let Some(win) = app.get_webview_window("sidebar") else { return };
        let Ok(Some(m)) = app.primary_monitor() else { return };
        let scale = m.scale_factor();
        let screen_w = m.size().to_logical::<f64>(scale).width;
        let slide = slide_mode(&app, &s.side);
        let on_left = s.side == "left";
        let (from, to) = if slide {
            let x = win.outer_position().map(|p| p.x as f64 / scale).unwrap_or(0.0);
            (x, slide_x(&s.side, s.window_width(), s.strip_width, screen_w, collapsed))
        } else {
            let w = win.outer_size().map(|z| z.width as f64 / scale).unwrap_or(s.window_width());
            (w, if collapsed { s.strip_width } else { s.window_width() })
        };
        const STEPS: u32 = 10;
        for i in 1..=STEPS {
            if app.state::<Runtime>().anim.load(Ordering::Relaxed) != gen {
                return; // superseded by a newer slide
            }
            let t = i as f64 / STEPS as f64;
            let v = from + (to - from) * (1.0 - (1.0 - t).powi(3));
            let win2 = win.clone();
            let _ = app.run_on_main_thread(move || {
                let Ok(ptr) = win2.ns_window() else { return };
                unsafe {
                    let ns: &objc2_app_kit::NSWindow = &*ptr.cast();
                    let mut f = ns.frame();
                    if slide {
                        f.origin.x = v;
                    } else {
                        let right = f.origin.x + f.size.width;
                        f.size.width = v;
                        if !on_left {
                            f.origin.x = right - v;
                        }
                    }
                    ns.setFrame_display(f, true);
                }
            });
            std::thread::sleep(Duration::from_millis(14));
        }
        if collapsed && app.state::<Runtime>().anim.load(Ordering::Relaxed) == gen {
            // The faded strip must never swallow clicks meant for the app underneath; the cursor poll
            // (not window events) brings the sidebar back at the screen edge.
            let _ = win.set_ignore_cursor_events(true);
            // No edge preview: leave nothing on screen (the screen edge still brings it back).
            if !s.edge_preview {
                let _ = win.hide();
            }
        }
    });
}

/// Puts the sidebar on the configured edge, applies float, hotkeys and login item.
fn apply(app: &AppHandle) {
    let s = app.state::<Runtime>().settings.lock().unwrap().clone();

    if let Some(win) = app.get_webview_window("sidebar") {
        let rt = app.state::<Runtime>();
        // Auto-hide switched off while collapsed: bring the full sidebar back.
        if !s.auto_hide && rt.auto_hidden.swap(false, Ordering::Relaxed) {
            let _ = app.emit("collapsed", false);
        }
        let collapsed = s.auto_hide && rt.auto_hidden.load(Ordering::Relaxed);
        place(app, &win, &s, collapsed);
        let _ = win.set_ignore_cursor_events(collapsed);
        if collapsed {
            let _ = if s.edge_preview { win.show() } else { win.hide() };
        }
        let _ = win.set_always_on_top(s.float_above);
        let _ = win.set_background_color(Some(Color(0, 0, 0, 0)));
    }

    let gs = app.global_shortcut();
    let _ = gs.unregister_all();
    if s.use_global_hotkeys {
        let bind = |keys: &str, f: fn(&AppHandle)| {
            let _ = gs.on_shortcut(keys, move |app, _, ev| {
                if ev.state == ShortcutState::Pressed {
                    f(app);
                }
            });
        };
        bind(&s.shortcuts.toggle, toggle_sidebar);
        bind(&s.shortcuts.settings, show_settings);
    }

    let _ = settings::set_launch_at_login(s.launch_at_login);
    let _ = app.emit("settings-changed", ());
}

/// Polls the cursor: reveals the sidebar at the screen edge, hides it after the cursor leaves.
fn spawn_auto_hide(app: AppHandle) {
    std::thread::spawn(move || {
        let mut left_at: Option<Instant> = None;
        let mut edge_at: Option<Instant> = None;
        loop {
            std::thread::sleep(Duration::from_millis(50));
            let rt = app.state::<Runtime>();
            let (auto_hide, side, reveal_delay) = {
                let s = rt.settings.lock().unwrap();
                (s.auto_hide, s.side.clone(), s.reveal_delay)
            };
            let Some(win) = app.get_webview_window("sidebar") else { continue };
            if !auto_hide {
                left_at = None;
                continue;
            }
            let (Ok(cur), Ok(pos), Ok(size), Ok(Some(mon))) =
                (app.cursor_position(), win.outer_position(), win.outer_size(), app.primary_monitor())
            else {
                continue;
            };
            let scale = mon.scale_factor();
            let screen_w = mon.size().width as f64;
            let at_edge = if side == "left" { cur.x <= 1.0 } else { cur.x >= screen_w - 2.0 };
            let inside = cur.x >= pos.x as f64
                && cur.x <= (pos.x + size.width as i32) as f64
                && cur.y >= pos.y as f64
                && cur.y <= (pos.y + size.height as i32) as f64;

            // The cursor has to stay on the screen edge for the configured delay.
            let edge_ready = if at_edge && cur.y > MENU_BAR * scale {
                edge_at.get_or_insert_with(Instant::now).elapsed() >= Duration::from_secs_f64(reveal_delay)
            } else {
                edge_at = None;
                false
            };

            if !win.is_visible().unwrap_or(false) {
                // Hidden with the hotkey/menu: the screen edge still brings it back.
                if edge_ready {
                    show_sidebar(&app);
                    edge_at = None;
                }
                continue;
            }
            if rt.auto_hidden.load(Ordering::Relaxed) {
                // Collapsed to a strip: only the screen edge itself (not hovering the strip)
                // expands the sidebar, so the browser scrollbar next to it stays usable.
                if edge_ready {
                    set_collapsed(&app, false);
                    edge_at = None;
                }
                left_at = None;
                continue;
            }
            let in_grace = Instant::now() < *rt.grace_until.lock().unwrap();
            if inside || in_grace || rt.pinned.load(Ordering::Relaxed) {
                left_at = None;
            } else if left_at.get_or_insert_with(Instant::now).elapsed() > Duration::from_millis(250) {
                set_collapsed(&app, true);
                left_at = None;
            }
        }
    });
}

/// Notifies for the reminders the events themselves carry (as set in the connected calendar), via osascript.
fn spawn_reminders(app: AppHandle) {
    std::thread::spawn(move || {
        let mut notified: std::collections::HashSet<String> = std::collections::HashSet::new();
        loop {
            std::thread::sleep(Duration::from_secs(15));
            let s = app.state::<Runtime>().settings.lock().unwrap().clone();
            if !calendar::has_access() {
                continue;
            }
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as f64)
                .unwrap_or(0.0);
            for e in calendar::events(now - 60_000.0, now + 2.0 * 86_400_000.0) {
                let hidden = s.hidden_events.contains(&e.id)
                    || s.calendars.iter().any(|c| c.id == e.calendar_id && !c.visible);
                if e.all_day || hidden {
                    continue;
                }
                for m in &e.alarms {
                    let fire = e.start - *m as f64 * 60_000.0;
                    // Only inside a one-minute window, so a wake from sleep doesn't replay old events.
                    if now < fire || now >= fire + 60_000.0 {
                        continue;
                    }
                    if !notified.insert(format!("{}@{}#{m}", e.id, e.start)) {
                        continue;
                    }
                    let mins = ((e.start - now) / 60_000.0).ceil().max(0.0) as i64;
                    let body = if mins == 0 { i18n::tr("Starting now", 0) } else { i18n::tr("Starts in {n} min", mins) };
                    let body = format!("{body} · {}", e.calendar);
                    let _ = std::process::Command::new("osascript")
                        .args([
                            "-e", "on run argv",
                            "-e", "display notification (item 1 of argv) with title (item 2 of argv)",
                            "-e", "end run",
                            &body, &e.title,
                        ])
                        .spawn();
                }
            }
            if notified.len() > 500 {
                notified.clear();
            }
        }
    });
}

fn main() {
    // `sidecal --mcp`: local MCP server over stdio for Claude and other MCP clients (no window, no network).
    if std::env::args().any(|a| a == "--mcp") {
        mcp::run();
        return;
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            connect_assistants,
            has_access,
            get_events,
            get_calendars,
            save_event,
            delete_event,
            open_event,
            set_title,
            set_pinned,
            get_settings,
            save_settings,
            set_event_color,
            set_event_hidden,
            open_settings,
            is_collapsed,
            slide_mode_enabled
        ])
        .setup(|app| {
            // Menu bar app: no Dock icon.
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let handle = app.handle().clone();
            let loaded = settings::load(&handle);
            // With auto-hide the app starts already collapsed (no collapse animation at launch).
            let start_collapsed = loaded.auto_hide;
            let start_slide = slide_mode(&handle, &loaded.side);
            app.manage(Runtime {
                settings: Mutex::new(loaded),
                pinned: AtomicBool::new(false),
                grace_until: Mutex::new(Instant::now()),
                auto_hidden: AtomicBool::new(start_collapsed),
                anim: AtomicU64::new(0),
            });

            WebviewWindowBuilder::new(app, "sidebar", WebviewUrl::App("index.html".into()))
                .title("SideCal")
                .decorations(false)
                .transparent(true)
                .shadow(false)
                .resizable(false)
                .skip_taskbar(true)
                .visible_on_all_workspaces(true)
                .background_color(Color(0, 0, 0, 0))
                // Handed to the page synchronously, before its scripts run (events could be missed).
                .initialization_script(format!(
                    "window.__SC = {{ collapsed: {start_collapsed}, slide: {start_slide} }};"
                ))
                .inner_size(190.0, 800.0)
                .build()?;

            let settings_item = MenuItem::with_id(app, "settings", &i18n::tr("Settings…", 0), true, Some("Cmd+,"))?;
            let update_item = MenuItem::with_id(app, "update", updater::CHECK_TITLE, true, None::<&str>)?;
            updater::init(&handle, update_item.clone());
            let quit = MenuItem::with_id(app, "quit", &i18n::tr("Quit SideCal", 0), true, None::<&str>)?;
            TrayIconBuilder::with_id(TRAY_ID)
                .title("Calendar")
                .menu(&Menu::with_items(app, &[&settings_item, &update_item, &quit])?)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, e| match e.id().as_ref() {
                    "settings" => show_settings(app),
                    "update" => updater::on_click(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, e| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                        toggle_sidebar(tray.app_handle());
                    }
                })
                .build(app)?;

            apply(&handle);
            calendar::request_access();
            updater::spawn_auto_check(handle.clone());
            spawn_reminders(handle.clone());
            spawn_auto_hide(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running SideCal");
}
