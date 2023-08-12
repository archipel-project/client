use bevy::prelude::*;
use bevy_egui::egui;
use egui_dock::{DockArea, NodeIndex, Style, Tree};
use egui_gizmo::GizmoMode;

use crate::app::types::EguiWindow;

use super::tab_viewer::TabViewer;

#[derive(Resource)]
pub struct UiState {
    tree: Tree<EguiWindow>,
    pub viewport_rect: egui::Rect,
    pub gizmo_mode: GizmoMode,

    pub should_quit: bool,
}

impl UiState {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![EguiWindow::GameView]);
        let [_game, inspector] =
            tree.split_left(NodeIndex::root(), 0.25, vec![EguiWindow::Inspector]);
        let [_inspector, _about] = tree.split_below(inspector, 0.5, vec![EguiWindow::About]);

        Self {
            tree,
            viewport_rect: egui::Rect::NOTHING,
            gizmo_mode: GizmoMode::Translate,
            should_quit: false,
        }
    }

    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        self.render_topbar_buttons(ctx);

        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            gizmo_mode: self.gizmo_mode,
        };

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(&ctx, &mut tab_viewer);

        self.render_bottombar_buttons(ctx);
    }

    fn render_topbar_buttons(&mut self, ctx: &mut egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        self.should_quit = true;
                    }
                });

                egui::menu::menu_button(ui, "Help", |ui| if ui.button("About").clicked() {});
            });
        });
    }

    fn render_bottombar_buttons(&mut self, ctx: &mut egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
