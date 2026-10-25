//! User settings, stored as JSON in the app config dir.
// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CalendarPref {
    pub id: String,
    pub visible: bool,
    pub productive: bool,
    /// Palette id overriding the calendar's own colour (None = use the calendar's colour).
    pub color: Option<String>,
}

impl Default for CalendarPref {
    fn default() -> Self {
        Self { id: String::new(), visible: true, productive: false, color: None }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Shortcuts {
    pub toggle: String,
    pub new_event: String,
    pub settings: String,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            toggle: "Ctrl+Alt+Cmd+S".into(),
            new_event: "Ctrl+Alt+Cmd+N".into(),
            settings: "Ctrl+Alt+Cmd+Comma".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    pub launch_at_login: bool,
    /// Asks GitHub Pages for latest.json once a day. Off by default: SideCal makes no network calls unless asked.
    pub check_for_updates: bool,
    pub float_above: bool,
    pub auto_hide: bool,
    pub show_title_in_menu_bar: bool,
    pub show_duration_in_menu_bar: bool,
    pub show_glowing_border: bool,
    pub show_time_indicator: bool,
    pub use_global_hotkeys: bool,
    pub display_all_day_events: bool,
    pub automatic_scroll: bool,
    pub use_12_hour: bool,
    pub suggest_events: bool,
    /// "light" | "dark" | "system"
    pub theme: String,
    /// "left" | "right"
    pub side: String,
    pub width: f64,
    pub show_date: bool,
    pub show_header: bool,
    pub show_weekday: bool,
    /// Expanded sidebar floats with a transparent margin and rounded corners.
    pub island_mode: bool,
    /// Edge preview: the thin strip of faded entries left on the screen edge while auto-hidden.
    pub edge_preview: bool,
    pub strip_width: f64,
    /// 0.2..=1.0
    pub preview_opacity: f64,
    /// Ordered; calendars missing here are visible and not productive.
    pub calendars: Vec<CalendarPref>,
    pub shortcuts: Shortcuts,
    /// Custom per-event colors (event id -> #rrggbb); calendars can't colour single events.
    pub event_colors: HashMap<String, String>,
    /// Ids of events the user hid from the sidebar (all occurrences of a repeating event).
    pub hidden_events: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            launch_at_login: false,
            check_for_updates: false,
            float_above: true,
            auto_hide: true,
            show_title_in_menu_bar: true,
            show_duration_in_menu_bar: true,
            show_glowing_border: true,
            show_time_indicator: true,
            use_global_hotkeys: true,
            display_all_day_events: true,
            automatic_scroll: true,
            use_12_hour: false,
            suggest_events: true,
            theme: "system".into(),
            side: "right".into(),
            width: 190.0,
            show_date: true,
            show_header: true,
            show_weekday: true,
            island_mode: false,
            edge_preview: true,
            strip_width: 10.0,
            preview_opacity: 0.6,
            calendars: Vec::new(),
            shortcuts: Shortcuts::default(),
            event_colors: HashMap::new(),
            hidden_events: Vec::new(),
        }
    }
}

impl Settings {
    /// Keeps hand-edited or old values inside what the UI can render.
    pub fn sanitized(mut self) -> Self {
        self.strip_width = self.strip_width.clamp(7.0, 40.0);
        self.preview_opacity = self.preview_opacity.clamp(0.2, 1.0);
        self
    }

    /// Transparent margin around the island (logical px, each side).
    pub fn pad(&self) -> f64 {
        if self.island_mode { ISLAND_PAD } else { 0.0 }
    }

    /// Width of the native window: the sidebar plus the island margin.
    pub fn window_width(&self) -> f64 {
        self.width + 2.0 * self.pad()
    }
}

/// Island margin: wide enough to read as floating, small enough not to waste screen.
pub const ISLAND_PAD: f64 = 8.0;

fn path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_config_dir().ok().map(|d| d.join("settings.json"))
}

pub fn load(app: &AppHandle) -> Settings {
    path(app)
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str::<Settings>(&s).ok())
        .map(Settings::sanitized)
        .unwrap_or_default()
}

/// Loads the settings file without a running app (MCP mode). Same location as Tauri's app config dir.
pub fn load_standalone() -> Settings {
    std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join("Library/Application Support/com.kacpergorec.sidecal/settings.json"))
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str::<Settings>(&s).ok())
        .map(Settings::sanitized)
        .unwrap_or_default()
}

pub fn save(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let p = path(app).ok_or("No config dir")?;
    if let Some(dir) = p.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(p, json).map_err(|e| e.to_string())
}

/// Launch at login via a per-user LaunchAgent (no extra dependencies).
pub fn set_launch_at_login(enabled: bool) -> Result<(), String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let plist = PathBuf::from(home).join("Library/LaunchAgents/com.kacpergorec.sidecal.plist");
    if !enabled {
        let _ = std::fs::remove_file(plist);
        return Ok(());
    }
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key><string>com.kacpergorec.sidecal</string>
  <key>ProgramArguments</key><array><string>{}</string></array>
  <key>RunAtLoad</key><true/>
</dict>
</plist>
"#,
        exe.display().to_string().replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    );
    if let Some(dir) = plist.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(plist, body).map_err(|e| e.to_string())
}
