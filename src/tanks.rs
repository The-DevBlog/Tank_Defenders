use bevy::prelude::*;

use crate::{
    Barracks, CurrentAction, Enemy, Friendly, HealthbarBundle, Range, Target, UnitBundle,
    SPEED_QUANTIFIER,
};

pub struct TanksPlugin;

impl Plugin for TanksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank)
            .add_systems(Update, attack_if_in_radius);
    }
}

pub fn spawn_tank(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    barracks_q: Query<(&Transform, Entity), With<Barracks>>,
) {
    let mut tank = (
        UnitBundle::new(
            "Tank".to_string(),
            15.0 * SPEED_QUANTIFIER,
            50.0,
            125.0,
            1000.0,
            Vec3::new(4., 2., 6.),
            assets.load("audio/tank_fire.ogg"),
            Timer::from_seconds(1.5, TimerMode::Repeating),
            assets.load("tank.glb#Scene0"),
            Vec3::new(50.0, 0.0, 100.0),
        ),
        Enemy,
    );

    let mut tank_2 = (
        UnitBundle::new(
            "Tank".to_string(),
            15.0 * SPEED_QUANTIFIER,
            50.0,
            125.0,
            1000.0,
            Vec3::new(4., 2., 6.),
            assets.load("audio/tank_fire.ogg"),
            Timer::from_seconds(1.5, TimerMode::Repeating),
            assets.load("tank.glb#Scene0"),
            Vec3::new(20.0, 0.0, 150.0),
        ),
        Enemy,
    );

    if let Ok((barracks_transform, barracks_ent)) = barracks_q.get_single() {
        tank.0.destination.0 = Some(barracks_transform.translation);
        tank.0.target.0 = Some(barracks_ent);
        tank_2.0.destination.0 = Some(barracks_transform.translation);
        tank_2.0.target.0 = Some(barracks_ent);
    }

    let healthbar_height = 1.5;
    let healthbar_width = 10.0;
    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        healthbar_width,
        healthbar_height,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(
        healthbar_width,
        healthbar_height,
        Vec3::new(0.0, 10.0, 0.0),
        healthbar_img.clone(),
        healthbar_mesh.clone(),
    );

    let healthbar_2 = HealthbarBundle::new(
        healthbar_width,
        healthbar_height,
        Vec3::new(0.0, 10.0, 0.0),
        healthbar_img,
        healthbar_mesh,
    );

    cmds.spawn(tank).with_children(|parent| {
        parent.spawn(healthbar);
    });

    cmds.spawn(tank_2).with_children(|parent| {
        parent.spawn(healthbar_2);
    });
}

fn attack_if_in_radius(
    mut enemy_q: Query<(&Range, &Transform, &mut Target), With<Enemy>>,
    friendly_q: Query<(Entity, &Transform), With<Friendly>>,
    barracks_q: Query<Entity, With<Barracks>>,
) {
    for (range, enemy_transform, mut target) in enemy_q.iter_mut() {
        if let Ok(barracks_ent) = barracks_q.get_single() {
            target.0 = Some(barracks_ent);
        }

        for (friendly_ent, friendly_transform) in friendly_q.iter() {
            let distance = (enemy_transform.translation - friendly_transform.translation).length();
            if distance <= range.0 {
                target.0 = Some(friendly_ent);
            }
        }
    }
}
