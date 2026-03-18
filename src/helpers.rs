// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use core::f32;
use std::process::Command;

use eframe::{
    egui::{self, Align2, Color32, FontId, Key, Pos2, Stroke, Vec2, ViewportCommand},
    emath::Rot2,
    epaint::{self, PathShape, PathStroke},
};

use crate::app::Hring;

impl Hring {
    pub fn get_key(key: &str) -> Option<Key> {
        if let Some(key) = Key::from_name(key) {
            Some(key)
        } else {
            println!("bind '{key}' not match!");
            None
        }
    }

    pub fn exec_app(ctx: &eframe::egui::Context, exec_str: &str) {
        _ = Command::new("sh").arg("-c").arg(exec_str).spawn().ok();
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }

    pub fn draw_radar(&self, painter: &egui::Painter, center: Pos2, start: f32, end: f32) {
        let g = &self.graphic;

        let (color, stroke_color, stroke_width) = (
            Self::get_color32(g.radar_color),
            Self::get_color32(g.radar_stroke_color),
            g.radar_stroke_width,
        );

        painter.add(PathShape::convex_polygon(
            vec![
                center,
                center + Vec2::new(5000.0 * start.cos(), 5000.0 * -start.sin()),
                center + Vec2::new(5000.0 * end.cos(), 5000.0 * -end.sin()),
            ],
            color,
            PathStroke::new(stroke_width, stroke_color),
        ));
    }

    pub fn draw_segment(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        start: f32,
        end: f32,
        is_selected: bool,
        group_bind: String,
    ) {
        let g = &self.graphic;

        let (
            segment_color,
            segment_stroke_color,
            segment_bind_color,
            segment_bind_font_color,
            segment_bind_font_size,
            segment_stroke_width,
        ) = if is_selected {
            (
                Self::get_color32(g.segment_color_active),
                Self::get_color32(g.segment_stroke_color_active),
                Self::get_color32(g.segment_bind_color_active),
                Self::get_color32(g.segment_bind_font_color_active),
                g.segment_bind_font_size_active,
                g.segment_stroke_width_active,
            )
        } else {
            (
                Self::get_color32(g.segment_color_unactive),
                Self::get_color32(g.segment_stroke_color_unactive),
                Self::get_color32(g.segment_bind_color_unactive),
                Self::get_color32(g.segment_bind_font_color_unactive),
                g.segment_bind_font_size_unactive,
                g.segment_stroke_width_unactive,
            )
        };

        let mut points: Vec<Pos2> = Vec::new();

        let step = (end - start) / (f32::from(g.segment_points_count));

        points.push(center);

        for i in 0..=g.segment_points_count {
            points.push(
                center
                    + Vec2::new(
                        g.segment_radius * (start + (step * f32::from(i))).cos(),
                        g.segment_radius * -(start + (step * f32::from(i))).sin(),
                    ),
            );
        }

        painter.add(PathShape::convex_polygon(
            points,
            segment_color,
            Stroke::new(segment_stroke_width, segment_stroke_color),
        ));

        let mid_rad = start + (end - start) / 2.0;
        let bind_pos = center
            + Vec2::new(
                g.segment_radius * mid_rad.cos(),
                g.segment_radius * -mid_rad.sin(),
            );
        painter.circle_filled(bind_pos, g.segment_bind_radius, segment_bind_color);
        painter.text(
            bind_pos,
            Align2::CENTER_CENTER,
            group_bind,
            FontId::monospace(segment_bind_font_size),
            segment_bind_font_color,
        );
    }

    pub fn draw_line(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        center_rad: f32,
        app_rad: f32,
        is_selected: bool,
    ) {
        let g = &self.graphic;

        let p0 = center
            + Vec2::new(
                g.app_offset * g.line_point_scale_1 * center_rad.cos(),
                g.app_offset * g.line_point_scale_1 * -center_rad.sin(),
            );

        let p1 = center
            + Vec2::new(
                g.app_offset * g.line_point_scale_2 * center_rad.cos(),
                g.app_offset * g.line_point_scale_2 * -center_rad.sin(),
            );

        let p2 = center
            + Vec2::new(
                g.app_offset * g.line_point_scale_3 * app_rad.cos(),
                g.app_offset * g.line_point_scale_3 * -app_rad.sin(),
            );

        let p3 = center + Vec2::new(g.app_offset * app_rad.cos(), g.app_offset * -app_rad.sin());

        let (line_color, line_width) = if is_selected {
            (Self::get_color32(g.line_color_active), g.line_width_active)
        } else {
            (
                Self::get_color32(g.line_color_unactive),
                g.line_width_unactive,
            )
        };

        painter.line_segment([center, p0], Stroke::new(line_width, line_color));

        painter.add(epaint::CubicBezierShape::from_points_stroke(
            [p0, p1, p2, p3],
            false,
            Color32::TRANSPARENT,
            PathStroke::new(line_width, line_color),
        ));
    }

    pub fn draw_apps(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        crt_app_rad: f32,
        is_selected: bool,
        bind_text: String,
    ) {
        let g = &self.graphic;

        let (app_color, app_font_color, app_font_size) = if is_selected {
            (
                Self::get_color32(g.app_color_active),
                Self::get_color32(g.app_font_color_active),
                g.app_font_size_active,
            )
        } else {
            (
                Self::get_color32(g.app_color_unactive),
                Self::get_color32(g.app_font_color_unactive),
                g.app_font_size_unactive,
            )
        };

        let app_pos = center
            + Vec2::new(
                g.app_offset * crt_app_rad.cos(),
                g.app_offset * -crt_app_rad.sin(),
            );

        painter.circle_filled(app_pos, g.app_radius, app_color);

        painter.text(
            app_pos,
            Align2::CENTER_CENTER,
            bind_text,
            FontId::monospace(app_font_size),
            app_font_color,
        );
    }

    pub fn draw_app_text(
        &self,
        painter: &egui::Painter,
        app_name: &str,
        center: Pos2,
        app_rad: f32,
        is_selected: bool,
    ) {
        let g = &self.graphic;

        let (font_color, background_color, font_size) = if is_selected {
            (
                Self::get_color32(g.app_title_font_color_active),
                Self::get_color32(g.app_title_background_color_active),
                g.app_title_font_size_active,
            )
        } else {
            (
                Self::get_color32(g.app_title_font_color_unactive),
                Self::get_color32(g.app_title_background_color_unactive),
                g.app_title_font_size_unactive,
            )
        };

        let app_pos =
            center + Vec2::new(g.app_offset * app_rad.cos(), g.app_offset * -app_rad.sin());

        let galley = painter.layout_no_wrap(
            app_name.to_string(),
            FontId::monospace(font_size),
            font_color,
        );

        let ray_direction = egui::vec2(app_rad.cos(), -app_rad.sin());

        let (text_angle, mut text_pos) = if app_rad.cos() < 0.0 {
            (
                -app_rad + f32::consts::PI,
                app_pos + ray_direction * (g.app_title_offset + galley.size().x),
            )
        } else {
            (-app_rad, app_pos + ray_direction * g.app_title_offset)
        };

        let text_rotation = Rot2::from_angle(text_angle);
        text_pos += text_rotation * egui::vec2(0.0, -galley.size().y / 2.0);

        let padding = egui::vec2(
            g.app_title_background_paddings.0,
            g.app_title_background_paddings.1,
        );
        let local_rect = egui::Rect::from_min_max(
            egui::pos2(-padding.x, -padding.y),
            egui::pos2(galley.size().x + padding.x, galley.size().y + padding.y),
        );

        let corners = [
            local_rect.left_top(),
            local_rect.right_top(),
            local_rect.right_bottom(),
            local_rect.left_bottom(),
        ];

        let bg_points: Vec<Pos2> = corners
            .iter()
            .map(|cornet| text_pos + text_rotation * cornet.to_vec2())
            .collect();

        painter.add(PathShape::convex_polygon(
            bg_points,
            background_color,
            PathStroke::NONE,
        ));

        painter.add(egui::epaint::TextShape {
            pos: text_pos,
            galley,
            underline: Stroke::NONE,
            fallback_color: font_color,
            override_text_color: None,
            opacity_factor: 1.0,
            angle: text_angle,
        });
    }

    pub fn get_color32(rgba: (u8, u8, u8, u8)) -> Color32 {
        Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}
