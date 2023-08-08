use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use self::{error::print_error, ui::app::MyApp};

mod error;
mod ui;
mod window;

pub fn run() {
    App::new()
        // Register resources
        .init_resource::<window::Settings>()
        .init_resource::<MyApp>()
        // Register plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Register startup systems
        .add_systems(Startup, window::set_icon.pipe(print_error))
        .add_systems(Startup, window::set_title)
        .add_systems(Startup, window::set_size)
        // Register update systems
        .add_systems(Update, window::get_size)
        .add_systems(Update, ui::app::ui_window_system)
        // Run the app
        .run()
}
