use bevy::prelude::{GlobalTransform, Mat4, Projection, With, World};
use bevy_egui::egui;

use super::{camera::MainCamera, types::EguiWindow};

pub struct TabViewer<'a> {
    pub world: &'a mut World,
    pub viewport_rect: &'a mut egui::Rect,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        match window {
            EguiWindow::GameView => {
                *self.viewport_rect = ui.clip_rect();

                // TODO: Draw game view
            }
            EguiWindow::Inspector => {
                // TODO: Draw inspector
            }
            EguiWindow::About => {
                ui.heading("About");
                ui.label("This is a demo of egui-dock.");
                // TODO: Draw about
            }
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::GameView)
    }
}
