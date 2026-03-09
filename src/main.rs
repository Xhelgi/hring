// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use eframe::egui;

mod app;
mod data;
mod theme;
mod ui;
mod utils;

use app::MyApp;

// * ============================================================================
// * 🏁 MAIN ENTRY POINT
// * ============================================================================

fn main() -> eframe::Result {
    // Configure window to be borderless, transparent, and always on top for a "launcher" feel.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_fullscreen(true)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "My Arch Launcher",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
