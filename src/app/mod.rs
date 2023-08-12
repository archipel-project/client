use bevy::{log::LogPlugin, prelude::*};
use bevy_egui::{EguiPlugin, EguiSet};

use self::{state::UiState, ui::show_ui_system, window::WindowSettings};

mod camera;
mod errors;
mod gizmo;
mod setup;
mod state;
mod tab_viewer;
mod types;
mod ui;
mod window;

pub fn run() {
    let mut app = App::new();

    // Register resources
    app.insert_resource(UiState::new())
        .register_type::<Option<Handle<Image>>>()
        .register_type::<AlphaMode>()
        .init_resource::<WindowSettings>();

    // Register plugins
    #[cfg(debug_assertions)]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,archipel_client=trace,wgpu_core=warn,wgpu_hal=warn".into(),
    }));
    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    }));
    app.add_plugins(EguiPlugin);

    // Register startup systems
    app.add_systems(PreStartup, window::set_icon.pipe(errors::print_error))
        .add_systems(PreStartup, window::set_title)
        .add_systems(PreStartup, window::set_size)
        .add_systems(Startup, setup::setup);

    // Register update systems
    app.add_systems(Update, window::get_size)
        .add_systems(Update, gizmo::set_gizmo_mode)
        .add_systems(Update, ui::exit_on_event)
        // Register post-update systems
        .add_systems(
            PostUpdate,
            ui::show_ui_system
                .before(EguiSet::ProcessOutput)
                .before(bevy::transform::TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            camera::set_camera_viewport.after(show_ui_system),
        );

    app.run();
}
