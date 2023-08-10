use bevy_egui::egui::{self, Context};

use super::traits::EguiWidget;

pub struct BottomBar {}

impl Default for BottomBar {
    fn default() -> Self {
        Self {}
    }
}

impl EguiWidget for BottomBar {
    fn show(&mut self, ctx: &Context) {
        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.show_menu(ui);
            })
        });
    }
}

impl BottomBar {
    fn show_menu(&mut self, ui: &mut egui::Ui) {
        egui::warn_if_debug_build(ui);
    }
}
