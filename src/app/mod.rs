use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use self::{error::print_error, ui::app::MyApp};

mod error;
mod events;
mod ui;
mod window;

pub fn run() {
    let mut app = App::new();
    // Register resources
    app.init_resource::<window::Settings>()
        .init_resource::<MyApp>();

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
    app.add_systems(Startup, window::set_icon.pipe(print_error))
        .add_systems(Startup, window::set_title)
        .add_systems(Startup, window::set_size);

    // Register update systems
    app.add_systems(Update, window::get_size)
        .add_systems(Update, ui::app::ui_window_system)
        .add_systems(Update, events::ui_events_system);

    // Run the app
    app.run()
}
