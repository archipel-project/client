use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use winit::window::Icon;

#[derive(Resource)]
pub struct WindowSettings {
    pub cur_height: f32,
    pub cur_width: f32,

    pub min_height: f32,
    pub min_width: f32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            cur_height: 400.0,
            cur_width: 600.0,

            min_height: 400.0,
            min_width: 600.0,
        }
    }
}

pub fn set_icon(
    entity: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(primary) = windows.get_window(entity.single()) {
        let img = image::open("resources/images/icon.ico")?.to_rgba8();
        let img_size = img.dimensions();

        let icon = Icon::from_rgba(img.into_raw(), img_size.0, img_size.1)?;
        primary.set_window_icon(Some(icon));
    }

    Ok(())
}

pub fn set_title(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window.get_single_mut() {
        window.title = "Archipel's client".to_string();
    }
}
pub fn set_size(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    settings: ResMut<WindowSettings>,
) {
    if let Ok(mut window) = window.get_single_mut() {
        window.resize_constraints.min_width = settings.min_width;
        window.resize_constraints.min_height = settings.min_height;
    }
}

pub fn get_size(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<WindowSettings>,
) {
    if let Ok(window) = window.get_single_mut() {
        settings.cur_height = window.height();
        settings.cur_width = window.width();
    }
}
