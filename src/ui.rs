// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use core::f32;

use eframe::egui::{
    self, Align2, FontId, Frame, Key, Response, RichText, ScrollArea, ViewportCommand,
};

use crate::app::Hring;

impl eframe::App for Hring {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !self.was_updated_from_config_loader
            && let Ok(groups) = self.from_config_loader.try_recv()
        {
            self.binds = groups;
            self.was_updated_from_config_loader = true;
        }

        if let Ok(apps) = self.from_search_worker.try_recv() {
            self.apps = apps;
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        let text_edit = self.create_left_panel(ctx);

        self.response_processing(&text_edit, ctx);

        self.create_main_panel(ctx);
    }
}

impl Hring {
    fn create_left_panel(&mut self, ctx: &eframe::egui::Context) -> Response {
        let g = &self.graphic;

        let mut app_to_execute = None;

        let response = egui::SidePanel::left("all_apps_panel")
            .min_width(g.left_panel_width)
            .max_width(g.left_panel_width)
            .frame(Frame::new().fill(Self::get_color32(g.left_panel_color)))
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(
                    RichText::new("All Programms")
                        .color(Self::get_color32(g.menu_items_font_color))
                        .size(g.menu_items_font_size),
                );
                ui.separator();

                let text_edit = ui.text_edit_singleline(&mut self.search_text);
                ui.add_space(10.0);

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for app in &self.apps {
                                let button_text = RichText::new(&app.name)
                                    .color(Self::get_color32(g.menu_items_font_color))
                                    .size(g.menu_items_font_size);

                                let btn = egui::Button::selectable(false, button_text)
                                    .fill(Self::get_color32(g.menu_items_hover_color));

                                if ui.add_sized([ui.available_width(), 20.0], btn).clicked() {
                                    app_to_execute = Some(app.exec.clone());
                                };
                            }
                        })
                    });

                text_edit
            })
            .inner;

        if let Some(exec_path) = app_to_execute {
            Self::exec_app(ctx, &exec_path);
        }

        response
    }

    fn response_processing(&mut self, text_edit: &Response, ctx: &eframe::egui::Context) {
        let enter_pressed = ctx.input(|i| i.key_pressed(Key::Enter));

        if text_edit.changed() {
            self.to_search_worker
                .send(self.search_text.clone())
                .expect("SearchThread not reachable!");
        }

        if enter_pressed {
            if text_edit.has_focus() || text_edit.lost_focus() {
                if self.search_text.is_empty() {
                    text_edit.surrender_focus();
                } else if let Some(app) = self.apps.first() {
                    Self::exec_app(ctx, &app.exec);
                }
            } else {
                text_edit.request_focus();
            }
        }

        if !text_edit.has_focus() {
            for (index, group) in self.binds.iter().enumerate() {
                if let Some(key) = Self::get_key(&group.bind)
                    && ctx.input(|i| i.key_pressed(key))
                {
                    self.selected_group = Some(index);
                }
    
                if self.selected_group == Some(index) {
                    group.apps.iter().for_each(|app| {
                        if let Some(key) = Self::get_key(&app.bind)
                            && ctx.input(|i| i.key_pressed(key))
                        {
                            Self::exec_app(ctx, &app.exec);
                        }
                    });
                }
            }
        }
    }

    fn create_main_panel(&mut self, ctx: &eframe::egui::Context) {
        let g = &self.graphic;

        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(Self::get_color32(g.main_panel_color)))
            .show(ctx, |ui| {
                let available_rect = ctx.available_rect();
                let painter = ui.painter().with_clip_rect(available_rect);
                let center = available_rect.center();

                if !self.binds.is_empty() {
                    let groups_count = self.binds.len();
                    let step_rad = f32::consts::TAU / groups_count as f32;

                    self.binds.iter().enumerate().for_each(|(index, group)| {
                        let is_selected = self.selected_group.is_some_and(|s| s == index);

                        let start_rad = step_rad * (index as f32);
                        let end_rad = start_rad + step_rad;

                        if !group.apps.is_empty() {
                            let apps_count = group.apps.len();
                            let center_rad = step_rad / 2.0 + start_rad;
                            let apps_start_deg = center_rad
                                - (((apps_count as i32 / 2) - 1) as f32 * g.apps_spacing_rad + {
                                    if apps_count % 2 == 1 {
                                        g.apps_spacing_rad
                                    } else {
                                        g.apps_spacing_rad / 2.0
                                    }
                                });

                            // Draw Radar
                            if is_selected {
                                self.draw_radar(&painter, center, start_rad, end_rad);
                            }

                            group.apps.iter().enumerate().for_each(|(i, app)| {
                                let crt_app_rad = apps_start_deg + g.apps_spacing_rad * i as f32;

                                // Draw Lines
                                self.draw_line(
                                    &painter,
                                    center,
                                    center_rad,
                                    crt_app_rad,
                                    is_selected,
                                );

                                // Draw AppText
                                self.draw_app_text(
                                    &painter,
                                    &app.name,
                                    center,
                                    crt_app_rad,
                                    is_selected,
                                );

                                // Draw Apps
                                self.draw_apps(
                                    &painter,
                                    center,
                                    crt_app_rad,
                                    is_selected,
                                    app.bind.clone(),
                                );
                            });
                        }

                        // Draw Segment
                        self.draw_segment(
                            &painter,
                            center,
                            start_rad,
                            end_rad,
                            is_selected,
                            group.bind.clone(),
                        );
                    });
                }

                // Draw CenterCircle
                painter.circle_filled(center, g.center_radius, Self::get_color32(g.center_color));
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    g.middle_text.clone(),
                    FontId::monospace(g.middle_text_size),
                    Self::get_color32(g.middle_text_color),
                );
            });
    }
}
