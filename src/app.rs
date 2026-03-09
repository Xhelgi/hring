// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use std::{collections::HashMap, sync::mpsc::{self, Receiver, Sender}, thread};
use eframe::egui::Key;

use crate::{
    data::{AppEntry, GApp, Group}, 
    theme::ThemeSettings, 
    utils::{get_all_apps, load_config}
};


// * ============================================================================
// * 🚀 APPLICATION STATE
// * ============================================================================

pub struct MyApp {
    // * UI Input State
    // ? Tracks real-time user input and the filtered list of apps currently visible.
    pub search_query: String,
    pub filtered_apps: Vec<AppEntry>,
    pub groups: Vec<Group>,
    pub selected_group_index: Option<usize>, // Tracks the currently "hoovered" or active donut sector.
    
    // * Settings
    // ? Maps string identifiers (from config) to actual egui::Key enum variants.
    pub key_map: HashMap<String, Key>,
    pub theme: ThemeSettings,
    
    // * Async Search Worker
    // ? Using Channels (mpsc) to handle search queries in a separate thread.
    // ? This prevents the UI from freezing while scanning large number of files.
    pub last_request_id: i32,
    pub last_result_id: i32,
    pub to_worker: Sender<(String, i32)>,             // Send search queries here.
    pub from_worker: Receiver<(Vec<AppEntry>, i32)>,  // Receive search results here.
}


// * ============================================================================
// * 🎯 MAIN APP IMPLEMENTATION
// * ============================================================================

impl MyApp {
    /// Initializes application state, scans the filesystem, loads config,
    /// and starts the search-filtering worker thread.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let all_apps = get_all_apps();
        let apps_copy = all_apps.clone();
        
        let config = load_config();

        // Join parsed JSON data with system app records to create "GApp" items.
        let mut groups = Vec::new();
        for saved_group in config.groups {
            let mut apps = Vec::new();
            for sapp in saved_group.apps {
                if let Some(app) = all_apps.iter()
                    .find(|a| a.name.to_lowercase() == sapp.name.to_lowercase()) {

                        apps.push(GApp { 
                            name: app.name.clone(), 
                            exec: app.exec.clone(), 
                            bind: sapp.bind 
                        });
                }
            }
            groups.push(Group {
                name: saved_group.name,
                apps,
                bind: saved_group.bind,
            });
        }

        // Spin up an asynchronous worker to handle potentially heavy filtering logic.
        let (tx_query, rx_query) = mpsc::channel::<(String, i32)>();
        let (tx_result, rx_result) = mpsc::channel::<(Vec<AppEntry>, i32)>();

        thread::spawn(move || {
            while let Ok((query, id)) = rx_query.recv() {
                let query_low = query.to_lowercase();
                let results: Vec<AppEntry> = all_apps
                    .iter()
                    .filter(|app| app.name.to_lowercase().contains(&query_low))
                    .cloned()
                    .collect();
                tx_result.send((results, id)).ok();
            }
        });

        // Hardcoded key-to-enum mapping.
        let mut key_map = HashMap::new();
        let keys =[
            ("q", Key::Q), ("w", Key::W), ("e", Key::E), ("a", Key::A), 
            ("s", Key::S), ("d", Key::D), ("z", Key::Z), ("x", Key::X), ("c", Key::C),
            ("1", Key::Num1), ("2", Key::Num2), ("3", Key::Num3), ("4", Key::Num4), ("5", Key::Num5)
        ];
        for (k, v) in keys {
            key_map.insert(k.to_string(), v);
        }

        Self {
            search_query: String::new(),
            filtered_apps: apps_copy,
            last_request_id: 0,
            last_result_id: 0,
            to_worker: tx_query,
            from_worker: rx_result,
            groups,
            selected_group_index: None,
            key_map,
            theme: ThemeSettings::default(),
        }
    }

    /// Spawns a shell-based command to launch an application.
    /// Uses `.ok()` to ensure errors don't terminate the launcher itself.
    pub fn launch_selected(&self, exec: &str) {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(exec)
            .spawn()
            .ok(); 
    }

    /// Safely lookup a configured key binding in the hashmap.
    pub fn get_key(&self, bind: &str) -> Option<Key> {
        self.key_map.get(&bind.to_lowercase()).copied()
    }
}