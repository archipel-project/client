use bevy::prelude::*;
use bevy_egui::egui::{self, Context, Id, Modifiers};

use super::traits::EguiWidget;

pub struct MenuBar {
    pixels_per_point: Option<f32>,
    show_debug_window: bool,
}

impl Default for MenuBar {
    fn default() -> Self {
        println!("test");
        Self {
            pixels_per_point: None,
            show_debug_window: false,
        }
    }
}

impl EguiWidget for MenuBar {
    fn show(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.show_menu(ui, ctx);
            })
        });
    }
}

impl MenuBar {
    fn show_menu(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let exit_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::Q);

        if ui.input_mut(|i| i.consume_shortcut(&exit_shortcut)) {
            self.exit_app(ui);
        }

        ui.menu_button("File", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);

            let quit_btn = ui.add(
                egui::Button::new("Exit").shortcut_text(ui.ctx().format_shortcut(&exit_shortcut)),
            );

            if quit_btn.clicked() {
                self.exit_app(ui);
                ui.close_menu();
            }
        });

        // On web, the browser controls the gui zoom.
        //println!("pixels_per_point: {:?}", ui.ctx().pixels_per_point());
        ui.menu_button("View", |ui| {
            egui::gui_zoom::zoom_menu_buttons(ui, self.pixels_per_point);

            ui.separator();

            let debug_btn = egui::Checkbox::new(&mut self.show_debug_window, "Show Debug Window");

            if ui.add(debug_btn).clicked() {
                self.toggle_debug_window(ui);

                ui.close_menu();
            }
        });
    }

    fn exit_app(&mut self, ui: &mut egui::Ui) {
        debug!("Exit app !");
        ui.memory_mut(|mem| mem.data.insert_temp("exit_app".into(), true));
    }

    fn toggle_debug_window(&mut self, ui: &mut egui::Ui) {
        ui.memory_mut(|mem| {
            let sdw_id = Id::new("show_debug_window");
            mem.data.insert_temp(sdw_id, self.show_debug_window);
        });
    }
}
