use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::{Health, HealthbarBundle, TankFactory, MAP_SIZE};

pub struct TankFactoryPlugin;

impl Plugin for TankFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank_factory);
    }
}

fn spawn_tank_factory(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let tank_factory = (
        SceneBundle {
            scene: assets.load("tank_factory.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(MAP_SIZE / 2.0 - 75.0, 16.5, 75.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, -8.3, 0.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(14.0, 20.0, 24.0),
        RigidBody::Fixed,
        TankFactory,
        Health::new(2000.0),
        Name::new("Tank Factory"),
    );

    let healthbar_width = 50.0;
    let healthbar_height = 2.5;
    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        healthbar_width,
        healthbar_height,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(
        healthbar_width,
        healthbar_height,
        Vec3::new(0.0, 30.0, 0.0),
        healthbar_img,
        healthbar_mesh,
    );

    cmds.spawn(tank_factory).with_children(|parent| {
        parent.spawn(healthbar);
    });
}
