use bevy::prelude::{In, ResMut, Resource};
use bevy_egui::{egui, EguiContexts};
use egui_dock::{DockArea, Style, Tree};

#[derive(Resource)]
pub struct MyApp {
    tree: Tree<String>,
    tab_view: TabViewer,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = Tree::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree.split_left(egui_dock::NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);
        let [_, _] = tree.split_below(a, 0.7, vec!["tab4".to_owned()]);
        let [_, _] = tree.split_below(b, 0.5, vec!["tab5".to_owned()]);

        Self {
            tree,
            tab_view: TabViewer {},
        }
    }
}

impl MyApp {
    fn show(&mut self, ctx: &mut egui::Context) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tab_view);
    }
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}

pub fn ui_window_system(mut contexts: EguiContexts, mut app: ResMut<MyApp>) {
    let mut ctx = contexts.ctx_mut();

    app.show(&mut ctx);
}
