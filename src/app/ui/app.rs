use bevy::prelude::{ResMut, Resource};
use bevy_egui::{egui, EguiContexts};

use super::{bottom_bar::BottomBar, content::AppContent, menu_bar::MenuBar, traits::EguiWidget};

#[derive(Resource)]
pub struct MyApp {
    menu_bar: MenuBar,
    content: AppContent,
    bottom_bar: BottomBar,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            menu_bar: MenuBar::default(),
            content: AppContent::default(),
            bottom_bar: BottomBar::default(),
        }
    }
}

impl MyApp {
    fn show(&mut self, ctx: &mut egui::Context) {
        self.menu_bar.show(ctx);
        self.content.show(ctx);
        self.bottom_bar.show(ctx);
    }
}

pub fn ui_window_system(mut contexts: EguiContexts, mut app: ResMut<MyApp>) {
    let mut ctx = contexts.ctx_mut();

    app.show(&mut ctx);
}
