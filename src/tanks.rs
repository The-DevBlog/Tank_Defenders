use bevy::prelude::*;

use crate::{Barracks, Enemy, HealthbarBundle, UnitBundle, SPEED_QUANTIFIER};

pub struct TanksPlugin;

impl Plugin for TanksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank);
    }
}

pub fn spawn_tank(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    barracks_q: Query<(&Transform, Entity), With<Barracks>>,
) {
    let tank_scene = assets.load("tank.glb#Scene0");
    let mut tank = (
        UnitBundle::new(
            "Tank".to_string(),
            15.0 * SPEED_QUANTIFIER,
            50.0,
            125.0,
            Vec3::new(4., 2., 6.),
            1000.0,
            Timer::from_seconds(1.5, TimerMode::Repeating),
            tank_scene,
            Vec3::new(50.0, 0.0, 100.0),
        ),
        Enemy,
    );

    tank.0.audio.source = assets.load("audio/tank_fire.ogg");

    if let Ok((barracks_transform, barracks_ent)) = barracks_q.get_single() {
        tank.0.destination.0 = Some(barracks_transform.translation);
        tank.0.target.0 = Some(barracks_ent);
    }

    let healthbar_width = 10.0;
    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(healthbar_width, 1.5)));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(
        healthbar_width,
        Vec3::new(0.0, 10.0, 0.0),
        healthbar_img,
        healthbar_mesh,
    );

    cmds.spawn(tank).with_children(|parent| {
        parent.spawn(healthbar);
    });
}
