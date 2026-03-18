// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

mod converter;
mod file_loader;

use homedir::my_home;
use std::path::PathBuf;

use crate::data::{AppLink, BindsConfig, ConfGroup, GlobalConfig, Graphic, GraphicConfig, Group};

const GRAPHIC_CONFIG_FILENAME: &str = ".config/hring/graphic.toml";
const GLOBAL_CONFIG_FILENAME: &str = ".config/hring/config.toml";
const BINDS_CONFIG_FILENAME: &str = ".config/hring/binds.toml";

const APP_LINKS_CACHE_FILENAME: &str = ".cache/hring/app_links.bin";
const BINDS_CACHE_FILENAME: &str = ".cache/hring/binds.bin";

pub fn get_home_dir() -> PathBuf {
    my_home().unwrap().expect("Cannot get home dir!")
}

fn graphic_config_path() -> PathBuf {
    get_home_dir().join(GRAPHIC_CONFIG_FILENAME)
}
fn gloval_config_path() -> PathBuf {
    get_home_dir().join(GLOBAL_CONFIG_FILENAME)
}
fn binds_config_path() -> PathBuf {
    get_home_dir().join(BINDS_CONFIG_FILENAME)
}

fn app_links_cache_path() -> PathBuf {
    get_home_dir().join(APP_LINKS_CACHE_FILENAME)
}
fn binds_cache_path() -> PathBuf {
    get_home_dir().join(BINDS_CACHE_FILENAME)
}

pub fn get_graphic() -> Graphic {
    if let Some((config_string, _)) = file_loader::read_config_file(&graphic_config_path()) {
        let graphic_config: GraphicConfig = converter::convert_toml_in_structure(&config_string);
        graphic_config.graphic
    } else {
        let graphic_config = GraphicConfig::default();
        let config_string = converter::convert_struct_in_toml(&graphic_config);
        file_loader::create_config_file_from_string(&graphic_config_path(), &config_string);
        graphic_config.graphic
    }
}

pub fn get_global_config() -> GlobalConfig {
    let mut global_config = GlobalConfig::default();

    if let Some((config_string, _)) = file_loader::read_config_file(&gloval_config_path()) {
        println!("global from config");
        global_config = converter::convert_toml_in_structure(&config_string);
    } else {
        let global_config_string = converter::convert_struct_in_toml(&global_config);
        file_loader::create_config_file_from_string(&gloval_config_path(), &global_config_string);
        println!("global from default and write to config");
    }

    global_config
}

pub fn get_app_links_from_cache() -> Option<Vec<AppLink>> {
    if let Some((cache_vec, _)) = file_loader::read_cache_file(&app_links_cache_path()) {
        let app_links: Vec<AppLink> = converter::convert_cache_in_structure(&cache_vec);
        Some(app_links)
    } else {
        None
    }
}

pub fn get_binds_from_cache() -> Option<Vec<Group>> {
    if let Some((cache_string, _)) = file_loader::read_cache_file(&binds_cache_path()) {
        let groups: Vec<Group> = converter::convert_cache_in_structure(&cache_string);
        Some(groups)
    } else {
        None
    }
}

pub fn get_binds_from_config() -> Vec<ConfGroup> {
    let default_binds_config = BindsConfig::default();
    if let Some((config_string, _)) = file_loader::read_config_file(&binds_config_path()) {
        let binds_config: BindsConfig = converter::convert_toml_in_structure(&config_string);
        println!("groups from config");
        binds_config.groups
    } else {
        let binds_config_default_string = converter::convert_struct_in_toml(&default_binds_config);
        file_loader::create_config_file_from_string(
            &binds_config_path(),
            &binds_config_default_string,
        );
        println!("groups from default groups (and create a new config)");
        default_binds_config.groups
    }
}

pub fn create_new_cache_for_groups(groups: &Vec<Group>) {
    let cache_vec = converter::convert_struct_in_cache(groups);
    file_loader::create_cache_file_from_slice(&binds_cache_path(), &cache_vec);
}

pub fn create_new_cache_for_app_links(app_links: &Vec<AppLink>) {
    let cache_vec = converter::convert_struct_in_cache(app_links);
    file_loader::create_cache_file_from_slice(&app_links_cache_path(), &cache_vec);
}
