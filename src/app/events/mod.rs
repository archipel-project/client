use bevy::{
    app::AppExit,
    prelude::{EventWriter, ResMut},
};
use bevy_egui::{egui, EguiContexts};

use super::ui::app::MyApp;

pub fn ui_events_system(
    mut contexts: EguiContexts,
    app: ResMut<MyApp>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    let should_exit = ctx
        .memory_mut(|mem| mem.data.get_temp::<bool>("exit_app".into()))
        .unwrap_or(false);
    let show_debug_window = ctx
        .memory(|mem| mem.data.get_temp::<bool>("show_debug_window".into()))
        .unwrap_or(false);

    if should_exit {
        exit.send(AppExit);
    }

    if show_debug_window {
        egui::Window::new("Events").show(ctx, |ui| {
            ctx.inspection_ui(ui);
        });
    }
}
