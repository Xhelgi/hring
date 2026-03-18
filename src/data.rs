// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Encode, Decode)]
pub struct AppLink {
    pub name: String,
    pub exec: String,
}

#[derive(Debug, Encode, Decode)]
pub struct App {
    pub bind: String,
    pub name: String,
    pub exec: String,
}

#[derive(Debug, Encode, Decode)]
pub struct Group {
    pub bind: String,
    pub apps: Vec<App>,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Graphic {
    // Panels
    pub main_panel_color: (u8, u8, u8, u8),
    pub left_panel_color: (u8, u8, u8, u8),
    pub left_panel_width: f32,

    // Menu Panel
    pub menu_items_hover_color: (u8, u8, u8, u8),
    pub menu_items_font_color: (u8, u8, u8, u8),
    pub menu_items_font_size: f32,
    // Main Panel
    pub middle_text: String,
    pub center_color: (u8, u8, u8, u8),
    pub middle_text_color: (u8, u8, u8, u8),
    pub middle_text_size: f32,
    pub center_radius: f32,

    // Apps
    pub app_color_active: (u8, u8, u8, u8),
    pub app_color_unactive: (u8, u8, u8, u8),
    pub app_font_color_active: (u8, u8, u8, u8),
    pub app_font_color_unactive: (u8, u8, u8, u8),
    pub app_title_font_color_active: (u8, u8, u8, u8),
    pub app_title_font_color_unactive: (u8, u8, u8, u8),
    pub app_title_background_color_active: (u8, u8, u8, u8),
    pub app_title_background_color_unactive: (u8, u8, u8, u8),

    pub app_font_size_active: f32,
    pub app_font_size_unactive: f32,
    pub app_title_font_size_active: f32,
    pub app_title_font_size_unactive: f32,

    pub app_radius: f32,
    pub app_offset: f32,
    pub apps_spacing_rad: f32,
    pub app_title_offset: f32,
    pub app_title_background_paddings: (f32, f32),

    // Lines
    pub line_color_active: (u8, u8, u8, u8),
    pub line_color_unactive: (u8, u8, u8, u8),

    pub line_width_active: f32,
    pub line_width_unactive: f32,

    pub line_point_scale_1: f32,
    pub line_point_scale_2: f32,
    pub line_point_scale_3: f32,

    // Segments
    pub segment_color_active: (u8, u8, u8, u8),
    pub segment_color_unactive: (u8, u8, u8, u8),
    pub segment_stroke_color_active: (u8, u8, u8, u8),
    pub segment_stroke_color_unactive: (u8, u8, u8, u8),
    pub segment_bind_color_active: (u8, u8, u8, u8),
    pub segment_bind_color_unactive: (u8, u8, u8, u8),
    pub segment_bind_font_color_active: (u8, u8, u8, u8),
    pub segment_bind_font_color_unactive: (u8, u8, u8, u8),

    pub segment_stroke_width_active: f32,
    pub segment_stroke_width_unactive: f32,
    pub segment_bind_font_size_active: f32,
    pub segment_bind_font_size_unactive: f32,

    pub segment_radius: f32,
    pub segment_points_count: u16,
    pub segment_bind_radius: f32,

    // Radar
    pub radar_color: (u8, u8, u8, u8),
    pub radar_stroke_color: (u8, u8, u8, u8),

    pub radar_stroke_width: f32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GraphicConfig {
    pub graphic: Graphic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindsConfig {
    pub groups: Vec<ConfGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub pathes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfApp {
    pub bind: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfGroup {
    pub bind: String,
    pub apps: Vec<ConfApp>,
}
