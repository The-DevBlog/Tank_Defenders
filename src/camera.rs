use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

use crate::MAP_SIZE;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle::default(),
        RtsCamera {
            bounds: Aabb2d::new(Vec2::ZERO, Vec2::new(MAP_SIZE / 2.0, MAP_SIZE / 2.0)),
            min_angle: 60.0f32.to_radians(),
            height_max: 200.0,
            ..default()
        },
        RtsCameraControls {
            edge_pan_width: 0.01,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            pan_speed: 165.0,
            zoom_sensitivity: 0.2,
            ..default()
        },
    ));
}
