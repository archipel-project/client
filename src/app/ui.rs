use bevy::{
    app::AppExit,
    prelude::{EventWriter, Res, With, World},
    window::PrimaryWindow,
};
use bevy_egui::EguiContext;

use super::state::UiState;

pub fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };

    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut());
    });
}

pub fn exit_on_event(mut app_exit_events: EventWriter<AppExit>, ui_state: Res<UiState>) {
    if ui_state.should_quit {
        app_exit_events.send(AppExit);
    }
}
