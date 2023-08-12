use bevy::{prelude::*, render::camera::CameraProjection};
use bevy_egui::egui;
use egui_gizmo::{Gizmo, GizmoMode, GizmoOrientation};

use crate::app::camera::MainCamera;

use super::state::UiState;

pub fn set_gizmo_mode(input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    for (key, mode) in [
        (KeyCode::R, GizmoMode::Rotate),
        (KeyCode::T, GizmoMode::Translate),
        (KeyCode::S, GizmoMode::Scale),
    ] {
        if input.just_pressed(key) {
            ui_state.gizmo_mode = mode;
        }
    }
}

pub fn draw_gizmo(ui: &mut egui::Ui, world: &mut World, gizmo_mode: GizmoMode) {
    let (cam_transform, projection) = world
        .query_filtered::<(&GlobalTransform, &Projection), With<MainCamera>>()
        .single(world);
    let view_matrix = Mat4::from(cam_transform.affine().inverse());
    let projection_matrix = projection.get_projection_matrix();

    let transform = Transform::default();
    let model_matrix = transform.compute_matrix();

    let Some(result) = Gizmo::new("selected")
        .model_matrix(model_matrix.to_cols_array_2d())
        .view_matrix(view_matrix.to_cols_array_2d())
        .projection_matrix(projection_matrix.to_cols_array_2d())
        .orientation(GizmoOrientation::Local)
        .mode(gizmo_mode)
        .interact(ui)
    else {
        return;
    };

    println!("{:?}", result);
}
