//! Tiny translation table for the few native strings (tray menu, notifications).
// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

use std::sync::OnceLock;

/// First supported language in the macOS preferred-languages list, else "en".
fn lang() -> &'static str {
    static LANG: OnceLock<&'static str> = OnceLock::new();
    LANG.get_or_init(|| {
        let out = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleLanguages"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .unwrap_or_default();
        for tok in out.split(|c: char| !c.is_ascii_alphabetic() && c != '-').filter(|t| t.len() >= 2) {
            match &tok.to_ascii_lowercase()[..2] {
                "pl" => return "pl",
                "de" => return "de",
                "es" => return "es",
                "fr" => return "fr",
                "en" => return "en",
                _ => {}
            }
        }
        "en"
    })
}

/// `key` is the English text; `{n}` is replaced by `n`.
pub fn tr(key: &str, n: i64) -> String {
    let s = match (lang(), key) {
        ("pl", "Settings…") => "Ustawienia…",
        ("pl", "Quit SideCal") => "Zakończ SideCal",
        ("pl", "Starting now") => "Zaczyna się teraz",
        ("pl", "Starts in {n} min") => "Zaczyna się za {n} min",
        ("de", "Settings…") => "Einstellungen…",
        ("de", "Quit SideCal") => "SideCal beenden",
        ("de", "Starting now") => "Beginnt jetzt",
        ("de", "Starts in {n} min") => "Beginnt in {n} Min.",
        ("es", "Settings…") => "Ajustes…",
        ("es", "Quit SideCal") => "Salir de SideCal",
        ("es", "Starting now") => "Empieza ahora",
        ("es", "Starts in {n} min") => "Empieza en {n} min",
        ("fr", "Settings…") => "Réglages…",
        ("fr", "Quit SideCal") => "Quitter SideCal",
        ("fr", "Starting now") => "Commence maintenant",
        ("fr", "Starts in {n} min") => "Commence dans {n} min",
        _ => key,
    };
    s.replace("{n}", &n.to_string())
}
