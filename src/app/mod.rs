use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSet};

use self::state::UiState;

mod camera;
mod setup;
mod state;
mod tab_viewer;
mod types;
mod ui;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(UiState::new())
        .add_systems(Startup, setup::setup)
        .add_systems(
            PostUpdate,
            ui::show_ui_system
                .before(EguiSet::ProcessOutput)
                .before(bevy::transform::TransformSystem::TransformPropagate),
        )
        .register_type::<Option<Handle<Image>>>()
        .register_type::<AlphaMode>()
        .run();
}
