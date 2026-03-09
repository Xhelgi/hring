// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use eframe::{
    egui::{self, Align2, Color32, FontId, Key, Pos2, Sense},
    emath::Rot2,
    epaint::{PathShape, PathStroke},
};

use crate::{
    app::MyApp,
    data::Group,
    utils::{get_config_path, get_shape_from_degree},
};

// * ============================================================================
// * 🎨 EGUI RENDER LOOP
// * ============================================================================

impl eframe::App for MyApp {
    /// Main rendering update loop.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Check for incoming search results from the background worker.
        while let Ok((results, id)) = self.from_worker.try_recv() {
            // Only update if the result is fresher than the currently displayed one.
            if id >= self.last_result_id {
                self.last_result_id = id;
                self.filtered_apps = results;
            }
        }

        // 2. UI rendering phases.
        let text_edit_response = self.draw_side_panel(ctx);
        self.handle_input(ctx, &text_edit_response);
        self.draw_central_panel(ctx);
    }
}

// * ============================================================================
// * 🧩 UI COMPONENTS & DRAWING HELPERS
// * ============================================================================

impl MyApp {
    /// Renders the sidebar with search input and the list of available apps.
    pub fn draw_side_panel(&mut self, ctx: &egui::Context) -> egui::Response {
        let mut text_edit_response = None;

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(200.0)
            .frame(egui::Frame::NONE.fill(self.theme.panel_bg_dark))
            .show(ctx, |ui| {
                ui.label("All Programs:");
                ui.separator();

                // Handle search box logic.
                let response = ui.text_edit_singleline(&mut self.search_query);
                if response.changed() {
                    self.last_request_id += 1;
                    // Trigger async search by sending query via channel.
                    self.to_worker
                        .send((self.search_query.clone(), self.last_request_id))
                        .ok();
                }
                text_edit_response = Some(response);

                ui.separator();

                // Bottom layout for persistent UI elements (config button).
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);

                    if ui.button("⚙ Open Config").clicked() {
                        let path = get_config_path().join("config.json");
                        // Trigger OS to open config in system text editor and close app.
                        std::process::Command::new("xdg-open")
                            .arg(path)
                            .spawn()
                            .ok();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    ui.separator();

                    // Scroll area for the filtered app list.
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for app in &self.filtered_apps {
                                    let btn = ui.add_sized(
                                        [ui.available_width(), 0.0],
                                        egui::Button::selectable(false, &app.name),
                                    );
                                    if btn.clicked() {
                                        self.launch_selected(&app.exec);
                                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
                                }
                            });
                    });
                });
            });

        text_edit_response.unwrap()
    }

    /// Global input handler for navigation, shortcut triggers, and focus management.
    pub fn handle_input(&mut self, ctx: &egui::Context, text_edit_response: &egui::Response) {
        // Case: Close App (ESC)
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Case: Search box is focused or has captured the user's attention.
        if text_edit_response.has_focus() || text_edit_response.lost_focus() {
            if ctx.input(|i| i.key_pressed(Key::Enter)) {
                if self.search_query.is_empty() {
                    text_edit_response.surrender_focus();
                } else {
                    // Launch first search match.
                    if let Some(app) = self.filtered_apps.first() {
                        self.launch_selected(&app.exec);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            }
        }
        // Case: User is navigating the graphical menu.
        else {
            if ctx.input(|i| i.key_pressed(Key::Enter)) {
                text_edit_response.request_focus();
            }

            // Keyboard navigation for group selection.
            for (i, group) in self.groups.iter().enumerate() {
                if let Some(key) = self.get_key(&group.bind) {
                    if ctx.input(|i| i.key_pressed(key)) {
                        self.selected_group_index = Some(i);
                    }
                }
            }

            // Trigger app launching based on current group selection.
            if let Some(idx) = self.selected_group_index {
                if let Some(group) = self.groups.get(idx) {
                    for app in &group.apps {
                        if let Some(key) = self.get_key(&app.bind) {
                            if ctx.input(|i| i.key_pressed(key)) {
                                self.launch_selected(&app.exec);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Renders the central graphical interface.
    /// This orchestrates the drawing of the donut segments and their connected apps.
    pub fn draw_central_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.theme.panel_bg_light))
            .show(ctx, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                let (_, painter) = ui.allocate_painter(available_rect.size(), Sense::empty());

                let center = available_rect.center();
                let groups_count = self.groups.len();
                if groups_count == 0 {
                    return;
                }

                let step_rad = (2.0 * std::f32::consts::PI) / groups_count as f32;
                let mut crt_rad = 0.0;

                // Iterate through groups to render radial sectors.
                for (i, group) in self.groups.iter().enumerate() {
                    let end_rad = crt_rad + step_rad;
                    let is_selected = self.selected_group_index == Some(i);

                    // Radar sweep rendering.
                    self.draw_radar(&painter, center, crt_rad, end_rad, is_selected);

                    // Branching wire and node rendering.
                    self.draw_group_apps(&painter, center, group, crt_rad, step_rad, is_selected);

                    // Donut segment rendering.
                    self.draw_donut_segment(
                        &painter,
                        center,
                        crt_rad,
                        end_rad,
                        is_selected,
                        &group.bind,
                    );

                    crt_rad = end_rad;
                }

                // Decorative central greeting element.
                painter.circle_filled(center, self.theme.center_radius - 20.0, Color32::BLACK);
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    &self.theme.center_text,
                    FontId::monospace(self.theme.center_text_size),
                    Color32::WHITE,
                );
            });
    }

    /// Renders a radar-style sweep effect to visually highlight the active group.
    pub fn draw_radar(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        start_rad: f32,
        end_rad: f32,
        is_selected: bool,
    ) {
        let t = &self.theme;
        if is_selected {
            painter.add(get_shape_from_degree(
                center,
                start_rad,
                end_rad,
                1000.0,
                t.color_radar,
            ));
        }
    }

    /// Renders a single sector segment for the center "donut" menu.
    pub fn draw_donut_segment(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        start_rad: f32,
        end_rad: f32,
        is_selected: bool,
        bind: &str,
    ) {
        let t = &self.theme;
        let color = if is_selected {
            t.color_active
        } else {
            t.color_inactive
        };
        let current_radius = if is_selected {
            t.center_radius + t.donut_thickness
        } else {
            t.center_radius
        };

        painter.add(get_shape_from_degree(
            center,
            start_rad,
            end_rad,
            current_radius,
            color,
        ));

        let text_rad = start_rad + (end_rad - start_rad) / 2.0;
        let pos = center
            + egui::vec2(
                current_radius * f32::cos(text_rad),
                current_radius * -f32::sin(text_rad),
            );

        // Visual indicator for key binding on the sector.
        painter.circle_filled(pos, 16.0, Color32::WHITE);

        painter.text(
            pos,
            Align2::CENTER_CENTER,
            bind,
            FontId {
                size: t.bind_text_size,
                family: egui::FontFamily::Monospace,
            },
            Color32::BLACK,
        );
    }

    /// Calculates orbital positions and draws branching Bézier wires connecting to application nodes.
    pub fn draw_group_apps(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        group: &Group,
        start_rad: f32,
        step_rad: f32,
        is_selected: bool,
    ) {
        let t = &self.theme;
        let apps_count = group.apps.len() as f32;
        if apps_count == 0.0 {
            return;
        }

        let sector_center_rad = start_rad + (step_rad / 2.0);
        let spacing_rad = t.app_spacing_deg.to_radians();
        let start_app_rad = sector_center_rad - (spacing_rad * (apps_count - 1.0) / 2.0);

        for (j, app) in group.apps.iter().enumerate() {
            let crt_app_rad = start_app_rad + (spacing_rad * j as f32);
            let orbit_radius = t.center_radius * t.orbit_multiplier;
            let app_pos =
                center + egui::vec2(f32::cos(crt_app_rad), -f32::sin(crt_app_rad)) * orbit_radius;

            // Resolve colors based on current interactivity state.
            let circle_color = if is_selected {
                t.color_app_active
            } else {
                t.color_app_inactive
            };
            let line_color = if is_selected {
                t.color_line_active
            } else {
                t.color_line_inactive
            };
            let text_color = if is_selected {
                t.color_text_active
            } else {
                t.color_text_inactive
            };

            // --- BEZIER CONTROL POINTS ---
            // p0: Origin point of the wire.
            // p1: Control point defining the initial straight "trunk".
            // p2: Control point creating the curve towards the app.
            // p3: Terminal point on the app node.
            let p0 = center
                + egui::vec2(f32::cos(sector_center_rad), -f32::sin(sector_center_rad))
                    * (orbit_radius * t.p_start_length);
            let p1 = center
                + egui::vec2(f32::cos(sector_center_rad), -f32::sin(sector_center_rad))
                    * (orbit_radius * t.p_trunk_length);
            let p2 = center
                + egui::vec2(f32::cos(crt_app_rad), -f32::sin(crt_app_rad))
                    * (orbit_radius * t.p_branch_curve);
            let p3 = app_pos;

            let line_width = if is_selected {
                t.line_stroke_active_width
            } else {
                t.line_stroke_inactive_width
            };
            let stroke = egui::Stroke::new(line_width, line_color);

            // --- RENDER WIRE ---
            painter.line_segment([center, p0], stroke);
            let curve = egui::epaint::CubicBezierShape::from_points_stroke(
                [p0, p1, p2, p3],
                false,
                Color32::TRANSPARENT,
                stroke,
            );
            painter.add(curve);

            // Render text label and node icon.
            let background_color = if is_selected {
                t.color_text_background_active
            } else {
                t.color_text_background_inactive
            };
            self.draw_app_text(
                painter,
                app_pos,
                crt_app_rad,
                &app.name,
                text_color,
                background_color,
            );

            painter.circle_filled(app_pos, t.app_circle_radius, circle_color);
            painter.text(
                app_pos,
                Align2::CENTER_CENTER,
                app.bind.clone(),
                FontId {
                    size: t.bind_text_size,
                    family: egui::FontFamily::Monospace,
                },
                Color32::WHITE,
            );
        }
    }

    /// Renders app labels rotated to point away from the center.
    /// This includes a semi-transparent background plate to improve readability.
    pub fn draw_app_text(
        &self,
        painter: &egui::Painter,
        app_pos: Pos2,
        angle_rad: f32,
        text: &str,
        text_color: Color32,
        background_color: Color32,
    ) {
        let t = &self.theme;
        let galley = painter.layout_no_wrap(
            text.to_string(),
            FontId::proportional(t.font_size),
            text_color,
        );
        let is_left_side = f32::cos(angle_rad) < 0.0;
        let ray_dir = egui::vec2(f32::cos(angle_rad), -f32::sin(angle_rad));

        // Adjust text rotation angle based on side to ensure it remains legible.
        let text_angle = if is_left_side {
            -angle_rad + std::f32::consts::PI
        } else {
            -angle_rad
        };

        let mut text_pos = app_pos;
        if is_left_side {
            text_pos += ray_dir * (t.text_offset + galley.size().x);
        } else {
            text_pos += ray_dir * t.text_offset;
        }

        let rot_txt = Rot2::from_angle(text_angle);
        text_pos += rot_txt * egui::vec2(0.0, -galley.size().y / 2.0);

        // Calculate background plate rect corners in screen space and apply transformation.
        let padding = egui::vec2(t.text_background_padding_x, t.text_background_padding_y);
        let local_rect = egui::Rect::from_min_max(
            egui::pos2(-padding.x, -padding.y),
            egui::pos2(galley.size().x + padding.x, galley.size().y + padding.y),
        );

        let mut bg_points = Vec::with_capacity(4);
        let corners = [
            local_rect.left_top(),
            local_rect.right_top(),
            local_rect.right_bottom(),
            local_rect.left_bottom(),
        ];

        for corner in corners {
            let rotated_vec = rot_txt * corner.to_vec2();
            bg_points.push(text_pos + rotated_vec);
        }

        painter.add(PathShape::convex_polygon(
            bg_points,
            background_color,
            PathStroke::NONE,
        ));

        let mut text_shape = egui::epaint::TextShape::new(text_pos, galley, text_color);
        text_shape.angle = text_angle;
        painter.add(text_shape);
    }
}
