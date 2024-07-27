use bevy::prelude::*;

use crate::{
    resources::RoundInfo, AdvanceRound, Barracks, Enemy, EnemyDestroyedEv, HealthbarBundle,
    UnitBundle, MAP_SIZE, SPEED_QUANTIFIER, TANK_DMG, TANK_HEALTH, TANK_RANGE, TANK_REWARD,
    TANK_SPEED,
};

pub struct RoundsPlugin;

impl Plugin for RoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .observe(spawn_tanks)
            .observe(advance_round)
            .observe(reset_round);
    }
}

fn setup(mut cmds: Commands) {
    cmds.trigger(AdvanceRound);
}

fn advance_round(
    _trigger: Trigger<EnemyDestroyedEv>,
    mut round_info: ResMut<RoundInfo>,
    mut cmds: Commands,
) {
    println!("Enemy Destroyed");
    round_info.enemies_defeated += 1;

    let total_enemies = round_info.enemy_soldiers + round_info.enemy_tanks;
    if round_info.enemies_defeated >= total_enemies {
        println!("New Round!");
        round_info.new_round();
        cmds.trigger(AdvanceRound);
    }
}

fn reset_round(_trigger: Trigger<AdvanceRound>, mut round_info: ResMut<RoundInfo>) {
    round_info.enemies_defeated = 0;
}

fn spawn_tanks(
    _trigger: Trigger<AdvanceRound>,
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    barracks_q: Query<(&Transform, Entity), With<Barracks>>,
    round_info: Res<RoundInfo>,
) {
    let Ok((barracks_transform, barracks_ent)) = barracks_q.get_single() else {
        return;
    };

    // Define the center of the arc and the radius
    let arc_center = barracks_transform.translation;
    let radius = MAP_SIZE - 50.0;
    let arc_angle = std::f32::consts::PI / 4.0; // 45 degrees arc

    for i in 0..round_info.enemy_tanks {
        let angle =
            arc_angle * (i as f32 / (round_info.enemy_tanks as f32 - 1.0)) - arc_angle / 2.0;
        let x = arc_center.x - MAP_SIZE + 100.0;
        let z = arc_center.z + radius * angle.sin();
        let initial_position = Vec3::new(x, arc_center.y, z);

        let mut tank = (
            UnitBundle::new(
                TANK_REWARD,
                "Tank".to_string(),
                TANK_SPEED * SPEED_QUANTIFIER,
                TANK_DMG,
                TANK_RANGE,
                TANK_HEALTH,
                Vec3::new(4., 2., 6.),
                assets.load("audio/tank_fire.ogg"),
                Timer::from_seconds(1.5, TimerMode::Repeating),
                assets.load("tank.glb#Scene0"),
                initial_position,
            ),
            Enemy,
        );

        tank.0.destination.0 = Some(barracks_transform.translation);
        tank.0.target.0 = Some(barracks_ent);

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

        cmds.spawn(tank).with_children(|parent| {
            parent.spawn(healthbar);
        });
    }
}
