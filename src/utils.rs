// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use eframe::{
    egui::{self, Color32, Pos2},
    epaint::{PathShape, PathStroke},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::data::{AppEntry, LauncherConfig};

// * ============================================================================
// * 🛠️ UTILITY FUNCTIONS (File System & Parsing)
// * ============================================================================

/// Scans standard Linux directories for `.desktop` files.
/// This function aggregates all applications from system and user paths.
pub fn get_all_apps() -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let home = std::env::var("HOME").unwrap_or_default();

    // XDG standard locations for Linux desktop entries.
    let paths = [
        "/usr/share/applications".to_string(),
        "/usr/local/share/applications".to_string(),
        format!("{}/.local/share/applications", home),
    ];

    for path_str in paths {
        if let Ok(entries) = fs::read_dir(path_str) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Ensure we only process .desktop files.
                if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                    if let Some(app) = parse_desktop_file(&path) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    // Sort for usability and remove duplicates by name.
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps.dedup_by(|a, b| a.name == b.name);

    apps
}

/// Parses a desktop entry, extracting the Name and Exec fields.
/// Automatically handles Linux field codes (e.g., %f, %u) by stripping them.
pub fn parse_desktop_file(path: &Path) -> Option<AppEntry> {
    let content = fs::read_to_string(path).ok()?;
    let mut name = String::new();
    let mut exec = String::new();
    let mut is_hidden = false;

    for line in content.lines() {
        let line = line.trim();

        // Skip hidden or disabled entries.
        if line == "NoDisplay=true" || line == "Hidden=true" {
            is_hidden = true;
        }

        if line.starts_with("Name=") && name.is_empty() {
            name = line.strip_prefix("Name=")?.to_string();
        }

        // Parse Exec command and cleanup standard Linux placeholders.
        if line.starts_with("Exec=") && exec.is_empty() {
            let raw_exec = line.strip_prefix("Exec=")?;
            exec = raw_exec
                .replace("%f", "")
                .replace("%F", "")
                .replace("%u", "")
                .replace("%U", "")
                .replace("%c", "")
                .replace("%C", "")
                .trim()
                .to_string();
        }
    }

    if is_hidden || name.is_empty() || exec.is_empty() {
        None
    } else {
        Some(AppEntry { name, exec })
    }
}

/// Helper for retrieving the standardized configuration directory path.
pub fn get_config_path() -> PathBuf {
    let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_default());
    path.push(".config/hring");
    path
}

/// Optional utility to write current group structures to JSON.
#[allow(dead_code)]
pub fn save_config(config: &LauncherConfig) -> anyhow::Result<()> {
    let dir = get_config_path();
    fs::create_dir_all(&dir)?;
    let path = dir.join("config.json");
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)?;
    Ok(())
}

/// Loads the user-defined configuration from `~/.config/my_launcher/config.json`.
pub fn load_config() -> LauncherConfig {
    let path = get_config_path().join("config.json");
    if !path.exists() {
        return LauncherConfig { groups: vec![] };
    }
    let content = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or(LauncherConfig { groups: vec![] })
}

// * ============================================================================
// * 📐 MATH UTILITIES
// * ============================================================================

/// Helper function generating a pie-slice polygon based on angular parameters.
/// Used for drawing the central navigation segments.
pub fn get_shape_from_degree(
    center: Pos2,
    start: f32,
    end: f32,
    radius: f32,
    color: Color32,
) -> PathShape {
    let mut points = vec![center];
    let steps = 20; // Number of segments to approximate circular arc.

    for i in 0..=steps {
        let t = start + (end - start) * (i as f32 / steps as f32);
        points.push(center + egui::vec2(f32::cos(t), -f32::sin(t)) * radius);
    }

    PathShape::convex_polygon(points, color, PathStroke::new(1.0, Color32::BLACK))
}
