use bevy::prelude::*;

use crate::{Enemy, HealthbarBundle, UnitBundle};

pub struct TanksPlugin;

impl Plugin for TanksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank);
    }
}

fn spawn_tank(mut cmds: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let tank_scene = assets.load("tank.glb#Scene0");
    let tank = (
        UnitBundle::new(
            "Tank".to_string(),
            5000.0,
            50,
            Vec3::new(4., 2., 6.),
            1000,
            tank_scene,
            Vec3::new(50.0, 0.0, 50.0),
        ),
        Enemy,
    );

    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        tank.0.scene_bundle.transform.scale.x * 10.0,
        1.5,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(Vec3::new(0.0, 10.0, 0.0), healthbar_img, healthbar_mesh);

    cmds.spawn(tank).with_children(|parent| {
        parent.spawn(healthbar);
    });
}
