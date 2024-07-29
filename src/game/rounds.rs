use super::*;
use crate::game::components::*;
use crate::game::events::*;
use crate::game::resources::*;
use bevy::prelude::*;

pub struct RoundsPlugin;

impl Plugin for RoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (ready_up_click, count_down_to_next_round))
            .observe(spawn_tanks)
            .observe(spawn_soldiers)
            .observe(advance_round)
            .observe(reset_round);
    }
}

fn setup(mut cmds: Commands) {
    let round_txt = (
        TextBundle {
            text: Text::from_section(
                "ROUND 1",
                TextStyle {
                    color: Color::srgb(0.0, 0.0, 0.0),
                    font_size: 50.0,
                    ..default()
                },
            ),
            style: Style {
                top: Val::Percent(0.5),
                right: Val::Percent(0.5),
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Percent(0.5)),
                ..default()
            },
            ..default()
        },
        RoundTxt,
        Name::new("Round Txt"),
    );

    cmds.spawn(round_txt);
    cmds.trigger(AdvanceRound);
}

fn advance_round(
    _trigger: Trigger<EnemyDestroyedEv>,
    mut round_info: ResMut<RoundInfo>,
    mut round_txt_q: Query<&mut Text, With<RoundTxt>>,
    mut cmds: Commands,
) {
    println!("Enemy Destroyed");
    round_info.enemies_defeated += 1;

    let total_enemies = round_info.enemy_soldiers + round_info.enemy_tanks;
    if round_info.enemies_defeated >= total_enemies {
        // update round display
        if let Ok(mut round_txt) = round_txt_q.get_single_mut() {
            round_txt.sections[0].value = format!("ROUND {}", round_info.round);
        }

        println!("New Round!");
        cmds.trigger(AdvanceRound);
    }
}

fn reset_round(_trigger: Trigger<AdvanceRound>, my_assets: Res<MyAssets>, mut cmds: Commands) {
    let ready_up_container = (
        ButtonBundle {
            image: UiImage::new(my_assets.img_hud_btn.clone()),
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(15.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.5), Val::Auto),
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        },
        ReadyUpBtn,
        Name::new("Ready Up Container"),
    );

    let ready_up_txt = (
        TextBundle {
            text: Text::from_section(
                "READY UP",
                TextStyle {
                    color: Color::srgb(0.0, 0.0, 0.0),
                    font_size: 50.0,
                    ..default()
                },
            ),
            style: Style {
                margin: UiRect {
                    top: Val::Auto,
                    bottom: Val::Auto,
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        ReadyUpTxt,
        Name::new("Text"),
    );

    cmds.spawn(ready_up_container).with_children(|parent| {
        parent.spawn(ready_up_txt);
    });
}

fn spawn_tanks(
    _trigger: Trigger<StartRound>,
    mut cmds: Commands,
    assets: Res<AssetServer>,
    my_assets: Res<MyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    barracks_q: Query<(&Transform, Entity), With<Barracks>>,
    round_info: Res<RoundInfo>,
) {
    let Ok((barracks_transform, barracks_ent)) = barracks_q.get_single() else {
        return;
    };

    let num_tanks = round_info.enemy_tanks as usize;
    let spacing = MAP_SIZE / (num_tanks as f32 + 1.0); // Calculate spacing between tanks

    for i in 0..num_tanks {
        // Calculate the x and z positions for the tank
        let x = -MAP_SIZE / 2.0 + 20.0; // Fixed x position on the left side of the map
        let z = -MAP_SIZE / 2.0 + spacing * (i as f32 + 1.0); // Spaced evenly along the z-axis
        let initial_position = Vec3::new(x, 2.0, z);

        let mut tank = (
            UnitBundle::new(
                TANK_REWARD,
                "Tank Enemy".to_string(),
                TANK_SPEED * SPEED_QUANTIFIER,
                TANK_DMG,
                TANK_RANGE,
                TANK_HEALTH,
                Vec3::new(4., 2., 6.),
                my_assets.audio_tank_fire.clone(),
                Timer::from_seconds(TANK_FIRE_RATE, TimerMode::Repeating),
                assets.load("tank_enemy.glb#Scene0"),
                initial_position,
            ),
            EnemyTank,
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

fn spawn_soldiers(
    _trigger: Trigger<StartRound>,
    mut cmds: Commands,
    assets: Res<AssetServer>,
    my_assets: Res<MyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    barracks_q: Query<(&Transform, Entity), With<Barracks>>,
    round_info: Res<RoundInfo>,
) {
    let Ok((barracks_transform, barracks_ent)) = barracks_q.get_single() else {
        return;
    };

    let num_soldiers = round_info.enemy_soldiers as usize;
    let spacing = MAP_SIZE / (num_soldiers as f32 + 1.0); // Calculate spacing between tanks

    for i in 0..num_soldiers {
        // Calculate the x and z positions for the tank
        let x = -MAP_SIZE / 2.0 + 50.0; // Fixed x position on the left side of the map
        let z = -MAP_SIZE / 2.0 + spacing * (i as f32 + 1.0); // Spaced evenly along the z-axis
        let initial_position = Vec3::new(x, 2.0, z);

        let mut soldier = (
            UnitBundle::new(
                SOLDIER_REWARD,
                "Soldier Enemy".to_string(),
                SOLDIER_SPEED * SPEED_QUANTIFIER,
                SOLDIER_DMG,
                SOLDIER_RANGE,
                SOLDIER_HEALTH,
                Vec3::new(2., 2., 2.),
                my_assets.audio_rifle_fire.clone(),
                Timer::from_seconds(SOLDIER_FIRE_RATE, TimerMode::Repeating),
                assets.load("soldier_animations.glb#Scene0"),
                initial_position,
            ),
            EnemySoldier,
            Enemy,
        );

        soldier.0.destination.0 = Some(barracks_transform.translation);
        soldier.0.target.0 = Some(barracks_ent);

        let healthbar_height = 1.0;
        let healthbar_width = 5.0;
        let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
            healthbar_width,
            healthbar_height,
        )));
        let healthbar_img = assets.load("imgs/full_health.png");
        let healthbar = HealthbarBundle::new(
            healthbar_width,
            healthbar_height,
            Vec3::new(0.0, 4.5, 0.0),
            healthbar_img.clone(),
            healthbar_mesh.clone(),
        );

        cmds.spawn(soldier).with_children(|parent| {
            parent.spawn(healthbar);
        });
    }
}

fn ready_up_click(
    mut interact_q: Query<&Interaction, (Changed<Interaction>, With<ReadyUpBtn>)>,
    mut round_info: ResMut<RoundInfo>,
) {
    for interaction in &mut interact_q {
        match *interaction {
            Interaction::Pressed => round_info.ready_up = true,
            _ => (),
        }
    }
}

fn count_down_to_next_round(
    mut round_info: ResMut<RoundInfo>,
    mut count_down_txt_q: Query<&mut Text, With<ReadyUpTxt>>,
    mut count_down_container_q: Query<Entity, With<ReadyUpBtn>>,
    time: Res<Time>,
    mut cmds: Commands,
) {
    if round_info.ready_up {
        if let Ok(mut count_down_txt) = count_down_txt_q.get_single_mut() {
            count_down_txt.sections[0].value =
                round_info.count_down.remaining_secs().ceil().to_string();

            if round_info.count_down.finished() {
                round_info.new_round();

                if let Ok(count_down_ent) = count_down_container_q.get_single_mut() {
                    cmds.entity(count_down_ent).despawn_recursive();
                }

                cmds.trigger(StartRound);
            }
        }

        round_info.count_down.tick(time.delta());
    }
}
