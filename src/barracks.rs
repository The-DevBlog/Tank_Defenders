use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

pub struct BarracksPlugin;

impl Plugin for BarracksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_barracks);
    }
}

fn spawn_barracks(mut cmds: Commands, assets: Res<AssetServer>) {
    let barracks = (
        SceneBundle {
            scene: assets.load("Barracks.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(110.0, 0.0, -80.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, -1.1, 0.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(16.0, 13.0, 16.0),
        RigidBody::Fixed,
        Name::new("Barracks"),
    );

    let healthbar = (
        NodeBundle {
            background_color: Color::srgb(0.0, 0.9, 0.0).into(),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            z_index: ZIndex::Local(1),
            ..default()
        },
        Name::new("Healthbar"),
    );

    cmds.spawn(barracks).with_children(|parent| {
        parent.spawn(healthbar);
    });
}
