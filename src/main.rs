// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

mod app;
mod config;
mod data;
mod data_impl;
mod helpers;
mod ui;

use app::Hring;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_always_on_top()
            .with_fullscreen(true)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Hring",
        options,
        Box::new(|_cc| Ok(Box::new(Hring::default()))),
    )
}
