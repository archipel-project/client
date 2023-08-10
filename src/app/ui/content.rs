use super::traits::EguiWidget;
use bevy_egui::egui;
use egui_dock::{DockArea, NodeIndex, Style, Tree};

pub struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}

pub struct AppContent {
    tree: Tree<String>,
    tab_viewer: TabViewer,
}

impl Default for AppContent {
    fn default() -> Self {
        let mut tree = Tree::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree.split_left(NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);
        let [_, _] = tree.split_below(a, 0.7, vec!["tab4".to_owned()]);
        let [_, _] = tree.split_below(b, 0.5, vec!["tab5".to_owned()]);

        let tab_viewer = TabViewer {};

        Self { tree, tab_viewer }
    }
}

impl EguiWidget for AppContent {
    fn show(&mut self, ctx: &egui::Context) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tab_viewer);
    }
}
