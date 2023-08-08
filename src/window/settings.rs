use bevy::winit::WinitWindows;
use bevy::{prelude::*, window::PrimaryWindow};
use winit::window::Icon;

#[derive(Resource)]
pub struct Settings {
    pub cur_height: f32,
    pub cur_width: f32,

    pub min_height: f32,
    pub min_width: f32,

    pub max_height: f32,
    pub max_width: f32,
}

impl Default for Settings {
    fn default() -> Self {
      Self {
        cur_height: 0.0,
        cur_width: 0.0,

        min_height: 400.0,
        min_width: 600.0,

        max_width: 800.0,
        max_height: 1200.0,
      }
    }
  }
 

pub fn set_icon(entity: Query<Entity, With<PrimaryWindow>>, windows: NonSend<WinitWindows>) -> Result<(), Box<dyn std::error::Error>> {
    let primary = windows.get_window(entity.single());
    if primary.is_none() { return Ok(()); }
    let primary = primary.unwrap();

    let img = image::open("resources/images/icon.ico")?.into_rgba8();
    let img_size = img.dimensions();

    let icon = Icon::from_rgba(img.into_raw(), img_size.0, img_size.1)?;
    primary.set_window_icon(Some(icon));

    Ok(())
}

pub fn set_title(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window.get_single_mut() {
        window.title = "Archipel's client".to_string();
    }
}

pub fn set_size(mut window: Query<&mut Window, With<PrimaryWindow>>, settings: ResMut<Settings>,) {
    if let Ok(mut window) = window.get_single_mut() {
        window.resize_constraints.min_width = settings.min_width;
        window.resize_constraints.min_height = settings.min_height;

        window.resize_constraints.max_width = settings.max_width;
        window.resize_constraints.max_height = settings.max_height;
    }
}

pub fn get_size(mut window: Query<&mut Window, With<PrimaryWindow>>, mut settings: ResMut<Settings>,) {
    if let Ok(window) = window.get_single_mut() {
        settings.cur_height = window.height();
        settings.cur_width = window.width();
    }
}
