use bevy::prelude::*;
use bevy_mod_billboard::{BillboardTextureBundle, BillboardTextureHandle};
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use crate::{
    resources::MyAssets, Barracks, BorderSelect, BuildSoldierEv, BuySoldierBtn, Friendly, Health,
    HealthbarBundle, PurchaseUnitRequestEv, Selected, Soldier, UnitBundle, UnitType, MAP_SIZE,
    SOLDIER_COST, SOLDIER_DMG, SOLDIER_FIRE_RATE, SOLDIER_HEALTH, SOLDIER_RANGE, SOLDIER_SPEED,
    SPEED_QUANTIFIER,
};

pub struct BarracksPlugin;

impl Plugin for BarracksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_barracks)
            .add_systems(Update, buy_soldier_click)
            .observe(build_soldier);
    }
}

fn spawn_barracks(mut cmds: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let barracks = (
        SceneBundle {
            scene: assets.load("barracks.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(MAP_SIZE / 2.0 - 75.0, 0.0, -75.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, -1.1, 0.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(16.0, 13.0, 16.0),
        RigidBody::Fixed,
        Barracks,
        Health::new(2000.0),
        Name::new("Barracks"),
    );

    let healthbar_width = 45.0;
    let healthbar_height = 2.5;
    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        healthbar_width,
        healthbar_height,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(
        healthbar_width,
        healthbar_height,
        Vec3::new(0.0, 22.5, 0.0),
        healthbar_img,
        healthbar_mesh,
    );

    cmds.spawn(barracks).with_children(|parent| {
        parent.spawn(healthbar);
    });
}

fn build_soldier(
    _trigger: Trigger<BuildSoldierEv>,
    barracks_q: Query<&Transform, With<Barracks>>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    my_assets: Res<MyAssets>,
    mut cmds: Commands,
) {
    let Ok(barracks_transform) = barracks_q.get_single() else {
        return;
    };

    let pos = barracks_transform.translation;
    let mut soldier = (
        UnitBundle::new(
            0,
            "Soldier".to_string(),
            SOLDIER_SPEED * SPEED_QUANTIFIER,
            SOLDIER_DMG,
            SOLDIER_RANGE,
            SOLDIER_HEALTH,
            Vec3::new(2., 2., 2.),
            my_assets.audio_rifle_fire.clone(),
            Timer::from_seconds(SOLDIER_FIRE_RATE, TimerMode::Repeating),
            assets.load("soldier_animations.glb#Scene0"),
            Vec3::new(pos.x - 30.0, 1.0, pos.z + 20.0),
        ),
        Selected(false),
        Soldier,
        Friendly,
    );

    soldier.0.destination.0 = Some(Vec3::new(pos.x - 100.0, 1.0, pos.z + 60.0));

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
        healthbar_img,
        healthbar_mesh,
    );

    let select_border = (
        BillboardTextureBundle {
            texture: BillboardTextureHandle(my_assets.select_border.clone()),
            ..default()
        },
        BorderSelect::new(7.5, 7.5),
        Name::new("Border Select"),
    );

    cmds.spawn(soldier).with_children(|parent| {
        parent.spawn(healthbar);
        parent.spawn(select_border);
    });

    println!("Building Soldier");
}

fn buy_soldier_click(
    mut cmds: Commands,
    mut interact_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuySoldierBtn>),
    >,
) {
    for (interaction, mut _background_clr) in &mut interact_q {
        match *interaction {
            Interaction::Pressed => {
                cmds.trigger(PurchaseUnitRequestEv::new(SOLDIER_COST, UnitType::Soldier))
            }
            _ => (),
        }
    }
}
