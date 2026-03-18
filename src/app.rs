// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use freedesktop_entry_parser::parse_entry;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::{
    config,
    data::{App, AppLink, Graphic, Group},
};

#[derive(Debug)]
pub struct Hring {
    pub apps: Vec<AppLink>,
    pub binds: Vec<Group>,
    pub graphic: Graphic,
    pub from_config_loader: Receiver<Vec<Group>>,
    pub to_search_worker: Sender<String>,
    pub from_search_worker: Receiver<Vec<AppLink>>,
    pub was_updated_from_config_loader: bool,
    pub search_text: String,
    pub selected_group: Option<usize>,
}

impl Default for Hring {
    fn default() -> Self {
        let graphic = config::get_graphic();
        let glocal_config = config::get_global_config();
        let apps = config::get_app_links_from_cache();
        let binds = config::get_binds_from_cache();

        let (sender_config_loader_to_update, receiver_update_from_config_loader): (
            Sender<Vec<Group>>,
            Receiver<Vec<Group>>,
        ) = mpsc::channel();

        let (sender_config_loader_to_search, receiver_search_from_config_loader): (
            Sender<Vec<AppLink>>,
            Receiver<Vec<AppLink>>,
        ) = mpsc::channel();

        let (sender_update_to_search, receiver_search_from_update): (
            Sender<String>,
            Receiver<String>,
        ) = mpsc::channel();

        let (sender_search_to_update, receiver_update_from_search): (
            Sender<Vec<AppLink>>,
            Receiver<Vec<AppLink>>,
        ) = mpsc::channel();

        // config_loader thread
        thread::spawn(move || {
            let (app_links, hash_map) = Self::search_app_links(glocal_config.pathes);
            let conf_groups = config::get_binds_from_config();

            let groups: Vec<Group> = conf_groups
                .into_iter()
                .map(|g| {
                    let apps: Vec<App> = g
                        .apps
                        .into_iter()
                        .filter_map(|a| {
                            if let Some(exec) = hash_map.get(&a.name.to_lowercase()) {
                                Some(App {
                                    bind: a.bind,
                                    name: a.name,
                                    exec: exec.clone(),
                                })
                            } else {
                                println!("App {} not fround!", a.name);
                                None
                            }
                        })
                        .collect();

                    Group { bind: g.bind, apps }
                })
                .collect();

            config::create_new_cache_for_groups(&groups);
            config::create_new_cache_for_app_links(&app_links);

            _ = sender_config_loader_to_update.send(groups);
            _ = sender_config_loader_to_search.send(app_links);
        });

        // search thread
        thread::spawn(move || {
            let mut was_updated_from_loader = false;
            let has_apps_from_cache = apps.is_some();

            let mut apps_from_loader: Vec<AppLink> = Vec::new();

            if has_apps_from_cache {
                apps_from_loader = apps.unwrap();
            }

            while let Ok(search_text) = receiver_search_from_update.recv() {
                if !was_updated_from_loader
                    && let Ok(new_apps_list) = receiver_search_from_config_loader.try_recv()
                {
                    apps_from_loader = new_apps_list;
                    was_updated_from_loader = true;
                }

                let search_text = search_text.to_ascii_lowercase();

                // searching
                if !was_updated_from_loader && !has_apps_from_cache {
                    _ = sender_search_to_update.send(Vec::new());
                    continue;
                }

                let filtred_apps: Vec<AppLink> = apps_from_loader
                    .iter()
                    .filter(|a| a.name.contains(&search_text))
                    .cloned()
                    .collect();

                _ = sender_search_to_update.send(filtred_apps);
            }
        });

        _ = sender_update_to_search.send(String::new());

        Hring {
            apps: Vec::new(),
            binds: binds.unwrap_or_default(),
            graphic,
            from_config_loader: receiver_update_from_config_loader,
            to_search_worker: sender_update_to_search,
            from_search_worker: receiver_update_from_search,
            was_updated_from_config_loader: false,
            search_text: String::new(),
            selected_group: None,
        }
    }
}

impl Hring {
    // TODO: To Rework
    fn search_app_links(pathes: Vec<String>) -> (Vec<AppLink>, HashMap<String, String>) {
        let mut apps: Vec<Option<AppLink>> = Vec::new();

        for path in pathes {
            if let Ok(file_pathes) = fs::read_dir(&path) {
                file_pathes
                    .into_iter()
                    .flatten()
                    .for_each(|p| apps.push(Self::parse_desktop_file(&p.path())));
            } else {
                println!("Path {path} cannot be read!");
            }
        }

        let mut apps: Vec<AppLink> = apps.into_iter().flatten().collect();

        apps.sort_by(|a, b| a.name.cmp(&b.name));
        apps.dedup_by(|a, b| a.name == b.name);

        let hash_map: HashMap<String, String> = apps
            .iter()
            .map(|a| (a.name.clone(), a.exec.clone()))
            .collect();

        (apps, hash_map)
    }

    // TODO: To Rework
    fn parse_desktop_file(path: &PathBuf) -> Option<AppLink> {
        let entry = parse_entry(path).ok()?;
        let section = entry.section("Desktop Entry")?;

        let name = section.attr("Name").first()?;
        let exec = section.attr("Exec").first()?;
        let no_display = section.attr("NoDisplay").first();

        let exec: String = exec
            .split_whitespace()
            .filter(|s| !s.contains('%'))
            .collect::<Vec<&str>>()
            .join(" ");

        if let Some(nd) = no_display
            && nd == "true"
        {
            return None;
        }

        Some(AppLink {
            name: name.clone().to_lowercase(),
            exec: exec.clone(),
        })
    }
}
