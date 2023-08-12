use bevy::{log::LogPlugin, prelude::*};
use bevy_egui::{EguiPlugin, EguiSet};
use bevy_fly_camera::FlyCameraPlugin;

use self::{
    ui::{show_ui_system, state::UiState},
    window::WindowSettings,
};

mod camera;
mod errors;
mod setup;
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
    app.add_plugins(FlyCameraPlugin);

    // Register startup systems
    app.add_systems(
        PreStartup,
        (
            window::set_icon.pipe(errors::print_error),
            window::set_title,
            window::set_size,
        ),
    )
    .add_systems(PreStartup, camera::setup_camera_fog)
    .add_systems(Startup, setup::setup);

    // Register pre-update systems
    app.add_systems(PreUpdate, camera::toggle_motion_system);

    // Register update systems
    app.add_systems(Update, window::get_size)
        .add_systems(Update, ui::gizmo::set_gizmo_mode)
        .add_systems(Update, ui::exit_on_event);

    // Register post-update systems
    app.add_systems(
        PostUpdate,
        ui::show_ui_system
            .before(EguiSet::ProcessOutput)
            .before(bevy::transform::TransformSystem::TransformPropagate),
    );
    // Register post-update systems
    app.add_systems(
        PostUpdate,
        camera::set_camera_viewport.after(show_ui_system),
    );

    app.run();
}
