// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use std::{fs, path::PathBuf, time::SystemTime};

pub fn read_config_file(path: &PathBuf) -> Option<(String, SystemTime)> {
    let content = fs::read_to_string(path).ok()?;
    let updated_time = fs::metadata(path).ok()?.modified().ok()?;
    Some((content, updated_time))
}

pub fn read_cache_file(path: &PathBuf) -> Option<(Vec<u8>, SystemTime)> {
    let content = fs::read(path).ok()?;
    let updated_time = fs::metadata(path).ok()?.modified().ok()?;
    Some((content, updated_time))
}

pub fn create_config_file_from_string(path: &PathBuf, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).expect("Cannot create a config directory!");
    fs::write(path, content).expect("Cannot write a config file!");
}

pub fn create_cache_file_from_slice(path: &PathBuf, content: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).expect("Cannot create a cache directory!");
    fs::write(path, content).expect("Cannot write a cache file!");
}
