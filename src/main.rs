use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use utils::error::print_error;

mod window;
mod models;
mod views;
mod utils;

fn main() {
    App::new()

    .init_resource::<utils::state::State>()
    .init_resource::<window::settings::Settings>()

    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)

    .add_systems(Startup, window::settings::set_icon.pipe(print_error))
    .add_systems(Startup, window::settings::set_title)
    .add_systems(Startup, window::settings::set_size)

    .add_systems(Update, window::settings::get_size)
    .add_systems(Update, views::connect::display_form)

    .run();
}
