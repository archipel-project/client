use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::camera::Viewport,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_egui::EguiSettings;
use bevy_fly_camera::FlyCamera;

use super::ui::state::UiState;

#[derive(Component)]
pub struct MainCamera;

pub fn set_camera_viewport(
    ui_state: Res<UiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
) {
    let mut cam = cameras.single_mut();

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor as f32;

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
}

pub fn setup_camera_fog(mut commands: Commands) {
    debug!("Setting up camera with fog system");
    let clear_color = Color::rgb_u8(128, 172, 255);

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(clear_color),
                ..Default::default()
            },
            ..Default::default()
        },
        FogSettings {
            color: clear_color,
            falloff: FogFalloff::Linear {
                start: 3.5,
                end: 10.0,
            },
            ..default()
        },
        FlyCamera {
            enabled: false,
            ..Default::default()
        },
        MainCamera,
    ));
}

pub fn toggle_motion_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut FlyCamera>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = primary_window.get_single_mut().unwrap();
    let mut camera = query.single_mut();

    if input.just_pressed(KeyCode::L) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
        camera.enabled = true;
    }

    if input.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
        camera.enabled = false;
    }
}
