use super::{MapBase, MAP_SIZE};
use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rts_camera::Ground;

// const MAP_SIZE: f32 = 100.0;
const WALL_THICKNESS: f32 = 1.0;
const WALL_HEIGHT: f32 = 10.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Walls
    let wall_material = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let wall_thickness_half = WALL_THICKNESS / 2.0;
    let wall_height_half = WALL_HEIGHT / 2.0;
    let half_map_size = MAP_SIZE / 2.0;

    // Spawn north wall
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(
                MAP_SIZE,
                WALL_HEIGHT,
                WALL_THICKNESS,
            ))),
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                0.0,
                wall_height_half,
                half_map_size + wall_thickness_half,
            ),
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, WALL_THICKNESS / 2.0),
        Name::new("North Wall"),
    ));

    // Spawn south wall
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(
                MAP_SIZE,
                WALL_HEIGHT,
                WALL_THICKNESS,
            ))),
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                0.0,
                wall_height_half,
                -(half_map_size + wall_thickness_half),
            ),
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, WALL_THICKNESS / 2.0),
        Name::new("South Wall"),
    ));

    // Spawn east wall
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(
                WALL_THICKNESS,
                WALL_HEIGHT,
                MAP_SIZE,
            ))),
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                half_map_size + wall_thickness_half,
                wall_height_half,
                0.0,
            ),
            ..default()
        },
        Collider::cuboid(WALL_THICKNESS / 2.0, WALL_HEIGHT / 2.0, MAP_SIZE / 2.0),
        Name::new("East Wall"),
    ));

    // Spawn west wall
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(
                WALL_THICKNESS,
                WALL_HEIGHT,
                MAP_SIZE,
            ))),
            material: wall_material,
            transform: Transform::from_xyz(
                -(half_map_size + wall_thickness_half),
                wall_height_half,
                0.0,
            ),
            ..default()
        },
        Collider::cuboid(WALL_THICKNESS / 2.0, WALL_HEIGHT / 2.0, MAP_SIZE / 2.0),
        Name::new("West Wall"),
    ));

    // Ground
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(MAP_SIZE, MAP_SIZE)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, 0.0, MAP_SIZE / 2.0),
        Ground,
        MapBase,
        Name::new("Map Base"),
    ));

    // Light
    cmds.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            150.0f32.to_radians(),
            -40.0f32.to_radians(),
            0.0,
        )),
        ..default()
    });
}
