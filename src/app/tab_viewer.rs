use bevy::prelude::World;
use bevy_egui::egui;
use egui_gizmo::GizmoMode;

use super::types::EguiWindow;

pub struct TabViewer<'a> {
    pub world: &'a mut World,
    pub viewport_rect: &'a mut egui::Rect,
    pub gizmo_mode: GizmoMode,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        match window {
            EguiWindow::GameView => {
                *self.viewport_rect = ui.clip_rect();

                // TODO: Draw game view
                super::gizmo::draw_gizmo(ui, self.world, self.gizmo_mode);
            }
            EguiWindow::Inspector => {
                // TODO: Draw inspector
                ui.heading("Inspector");
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
