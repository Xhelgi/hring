// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use crate::{
    config,
    data::{BindsConfig, ConfApp, ConfGroup, GlobalConfig, Graphic},
};

impl Default for Graphic {
    fn default() -> Self {
        Graphic {
            main_panel_color: (255, 255, 255, 10),
            left_panel_color: (20, 22, 20, 230),
            left_panel_width: 240.0,
            menu_items_hover_color: (92, 184, 122, 50),
            menu_items_font_color: (230, 230, 230, 255),
            menu_items_font_size: 14.0,
            middle_text: String::from(":Hring"),
            center_color: (15, 17, 15, 255),
            middle_text_color: (255, 255, 255, 255),
            middle_text_size: 14.0,
            center_radius: 40.0,
            app_color_active: (92, 184, 122, 255),
            app_color_unactive: (35, 38, 35, 255),
            app_font_color_active: (10, 12, 10, 255),
            app_font_color_unactive: (180, 180, 180, 255),
            app_title_font_color_active: (255, 255, 255, 255),
            app_title_font_color_unactive: (130, 130, 130, 255),
            app_title_background_color_active: (92, 184, 122, 180),
            app_title_background_color_unactive: (20, 22, 20, 180),
            app_font_size_active: 16.0,
            app_font_size_unactive: 14.0,
            app_title_font_size_active: 16.0,
            app_title_font_size_unactive: 14.0,
            app_radius: 22.0,
            app_offset: 230.0,
            apps_spacing_rad: 0.4,
            app_title_offset: 30.0,
            app_title_background_paddings: (16.0, 6.0),
            line_color_active: (92, 184, 122, 255),
            line_color_unactive: (10, 15, 10, 255),
            line_width_active: 3.0,
            line_width_unactive: 3.0,
            line_point_scale_1: 0.5,
            line_point_scale_2: 0.7,
            line_point_scale_3: 0.8,
            segment_color_active: (92, 184, 122, 255),
            segment_color_unactive: (25, 28, 25, 255),
            segment_stroke_color_active: (255, 255, 255, 200),
            segment_stroke_color_unactive: (45, 50, 45, 255),
            segment_bind_color_active: (255, 255, 255, 255),
            segment_bind_color_unactive: (40, 45, 40, 255),
            segment_bind_font_color_active: (10, 10, 10, 255),
            segment_bind_font_color_unactive: (120, 120, 120, 255),
            segment_stroke_width_active: 2.0,
            segment_stroke_width_unactive: 1.0,
            segment_bind_font_size_active: 14.0,
            segment_bind_font_size_unactive: 12.0,
            segment_radius: 60.0,
            segment_points_count: 32,
            segment_bind_radius: 12.0,
            radar_color: (92, 184, 122, 15),
            radar_stroke_color: (92, 184, 122, 40),
            radar_stroke_width: 1.0,
        }
    }
}

impl Default for BindsConfig {
    fn default() -> Self {
        BindsConfig {
            groups: vec![
                ConfGroup {
                    bind: "q".to_string(),
                    apps: vec![
                        ConfApp {
                            bind: "1".to_string(),
                            name: "Discord".to_string(),
                        },
                        ConfApp {
                            bind: "2".to_string(),
                            name: "Telegram".to_string(),
                        },
                    ],
                },
                ConfGroup {
                    bind: "w".to_string(),
                    apps: vec![
                        ConfApp {
                            bind: "1".to_string(),
                            name: "Discord_1".to_string(),
                        },
                        ConfApp {
                            bind: "2".to_string(),
                            name: "Telegram_1".to_string(),
                        },
                    ],
                },
            ],
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            pathes: vec![
                "/usr/share/applications".to_string(),
                "/usr/local/share/applications".to_string(),
                config::get_home_dir()
                    .join(".local/share/applications")
                    .to_str()
                    .unwrap()
                    .to_string(),
            ],
        }
    }
}
