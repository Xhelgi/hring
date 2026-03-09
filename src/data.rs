// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use serde::{Deserialize, Serialize};

// * ============================================================================
// * 📦 DATA MODELS
// * ============================================================================

// ? Raw structure retrieved directly from system .desktop files.
#[derive(Clone, Debug)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
}

// ? Representation of how an app is linked to a key in the user's config file.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedApp {
    pub name: String,
    pub bind: String,
}

// ? Groups are the primary organizational units in the launcher UI.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedGroup {
    pub name: String,
    pub apps: Vec<SavedApp>,
    pub bind: String,
}

// ? Root structure for deserializing the JSON configuration file.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LauncherConfig {
    pub groups: Vec<SavedGroup>,
}

// ? GApp (Graph App) is the enriched structure used by the renderer,
// ? combining the system executable path with the user-defined key binding.
#[derive(Debug)]
pub struct GApp {
    pub name: String,
    pub exec: String,
    pub bind: String,
}

// ? A resolved Group object ready for drawing on the canvas.
#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub apps: Vec<GApp>,
    pub bind: String,
}