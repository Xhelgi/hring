// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use eframe::egui::Color32;

// * ============================================================================
// * 🎨 UI THEME & SETTINGS
// * ============================================================================
// ! This structure acts as a central configuration hub for all visual parameters.
// ! By isolating these from the main logic, we ensure that design tweaks (colors, 
// ! sizes, animations) don't introduce bugs into the application behavior.

pub struct ThemeSettings {
    // * --- Text Blocks ---
    pub center_text: String, // Text displayed in the Mid

    // * --- Background Colors ---
    // ? Panel background colors handle the aesthetic transition between 
    // ? solid UI elements and the central interactive graph.
    pub panel_bg_dark: Color32,
    pub panel_bg_light: Color32,
    
    // * --- Donut (Center Graph) Colors ---
    // ? These control the visual status of the central navigation "donut" segments.
    pub color_active: Color32,
    pub color_inactive: Color32,
    pub color_radar: Color32, // The subtle sweep effect when a sector is selected
    
    // * --- Orbital Apps Colors ---
    // ? Visual state management for app nodes, including text and labels.
    pub color_app_active: Color32,
    pub color_app_inactive: Color32,
    pub color_text_active: Color32,
    pub color_text_inactive: Color32,
    pub color_text_background_active: Color32,
    pub color_text_background_inactive: Color32,
    
    // * --- Line Colors ---
    // ? Defines the stroke of the Bezier wires connecting the center to the apps.
    pub color_line_active: Color32,
    pub color_line_inactive: Color32,

    // * --- Geometry & Dimensions ---
    // ? These values define the layout grid of the launcher.
    pub center_radius: f32,       // The radius of the empty "black hole" center.
    pub donut_thickness: f32,     // Visual weight of the group navigation segments.
    pub orbit_multiplier: f32,    // Scaling factor for the distance of apps from center.
    pub app_circle_radius: f32,   // The radius of the icon node.
    pub app_spacing_deg: f32,     // Angular distance between app nodes to prevent overlaps.
    pub text_offset: f32,         // Padding between the node and its text label.
    pub text_background_padding_x: f32,
    pub text_background_padding_y: f32,

    // * --- Path Dynamics ---
    // ? Parameters controlling the "Bézier" wire generation.
    pub line_stroke_active_width: f32,
    pub line_stroke_inactive_width: f32,
    pub p_start_length: f32,    // The "Root" distance from the center.
    pub p_trunk_length: f32,    // Length of the segment before the curve starts.
    pub p_branch_curve: f32,    // Curve tension: influences how "sharp" the branching turn is.

    // * --- Text Sizes ---
    pub bind_text_size: f32,    // Font size for keyboard shortcut keys.
    pub font_size: f32,         // Global text size for application labels.
    pub center_text_size: f32,  // Font size for label in center "black hole".
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            // center_text: String::from("Hring"),

            // panel_bg_dark: Color32::from_black_alpha(200),
            // panel_bg_light: Color32::TRANSPARENT, 
            
            // color_active: Color32::GREEN,
            // color_inactive: Color32::from_gray(100),
            // color_radar: Color32::from_rgba_unmultiplied(200, 200, 180, 180),

            // color_app_active: Color32::DARK_GREEN,
            // color_app_inactive: Color32::from_gray(80),
            
            // color_text_active: Color32::WHITE,
            // color_text_inactive: Color32::from_gray(120),
            
            // color_text_background_active: Color32::from_black_alpha(180),
            // color_text_background_inactive: Color32::from_black_alpha(100),

            // color_line_active: Color32::from_rgb(100, 200, 100),
            // color_line_inactive: Color32::from_gray(80),
            
            // center_radius: 60.0,
            // donut_thickness: 10.0,
            // orbit_multiplier: 4.0,
            // app_circle_radius: 20.0,
            // app_spacing_deg: 20.0,
            // text_offset: 25.0,
            // text_background_padding_x: 12.0,
            // text_background_padding_y: 4.0,
            // line_stroke_active_width: 5.0,
            // line_stroke_inactive_width: 3.0,
            // p_start_length: 0.4,    
            // p_trunk_length: 0.7,    
            // p_branch_curve: 0.6,
            
            // bind_text_size: 18.0,
            // font_size: 20.0,
            // center_text_size: 18.0,
            center_text: String::from("Hring"),

            panel_bg_dark: Color32::from_black_alpha(200),
            panel_bg_light: Color32::from_rgba_unmultiplied(60, 30, 30, 70),
            
            color_active: Color32::DARK_RED,
            color_inactive: Color32::from_rgb(120, 110, 110),
            color_radar: Color32::from_rgba_unmultiplied(200, 180, 180, 180),

            color_app_active: Color32::RED,
            color_app_inactive: Color32::from_gray(80),
            
            color_text_active: Color32::DARK_RED,
            color_text_inactive: Color32::WHITE,
            
            color_text_background_active: Color32::from_rgba_unmultiplied(255, 255, 255, 180),
            color_text_background_inactive: Color32::from_rgba_unmultiplied(155, 155, 155, 180),

            color_line_active: Color32::from_rgb(230, 80, 80),
            color_line_inactive: Color32::from_rgb(150, 60, 60),
            
            center_radius: 60.0,
            donut_thickness: 10.0,
            orbit_multiplier: 4.0,
            app_circle_radius: 20.0,
            app_spacing_deg: 20.0,
            text_offset: 25.0,
            text_background_padding_x: 12.0,
            text_background_padding_y: 4.0,
            line_stroke_active_width: 5.0,
            line_stroke_inactive_width: 3.0,
            p_start_length: 0.4,    
            p_trunk_length: 0.7,    
            p_branch_curve: 0.6,
            
            bind_text_size: 18.0,
            font_size: 20.0,
            center_text_size: 18.0,
        }
    }
}
