use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use crate::{
    tanks::spawn_tank, Barracks, BuildUnitEv, BuySoldierBtn, Friendly, Health, HealthbarBundle,
    PurchaseSoldierRequestEv, UnitBundle, SPEED_QUANTIFIER,
};

pub struct BarracksPlugin;

impl Plugin for BarracksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_barracks.before(spawn_tank))
            .add_systems(Update, buy_soldier_click)
            .observe(build_unit);
    }
}

fn spawn_barracks(mut cmds: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let barracks = (
        SceneBundle {
            scene: assets.load("barracks.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(110.0, 0.0, -80.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, -1.1, 0.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(16.0, 13.0, 16.0),
        RigidBody::Fixed,
        Barracks,
        Health::new(500.0),
        Name::new("Barracks"),
    );

    let healthbar_width = 45.0;
    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(healthbar_width, 2.5)));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(
        healthbar_width,
        Vec3::new(0.0, 22.5, 0.0),
        healthbar_img,
        healthbar_mesh,
    );

    cmds.spawn(barracks).with_children(|parent| {
        parent.spawn(healthbar);
    });
}

fn build_unit(
    _trigger: Trigger<BuildUnitEv>,
    barracks_q: Query<&Transform, With<Barracks>>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cmds: Commands,
) {
    let Ok(barracks_transform) = barracks_q.get_single() else {
        return;
    };

    let pos = barracks_transform.translation;
    let mut soldier = (
        UnitBundle::new(
            "Soldier".to_string(),
            5.0 * SPEED_QUANTIFIER,
            5.0,
            50.0,
            Vec3::new(2., 2., 2.),
            50.0,
            Timer::from_seconds(0.25, TimerMode::Repeating),
            assets.load("soldier_animations.glb#Scene0"),
            Vec3::new(pos.x - 30.0, 1.0, pos.z + 20.0),
        ),
        Friendly,
    );

    soldier.0.audio.source = assets.load("audio/rifle_fire.ogg");
    soldier.0.destination.0 = Some(Vec3::new(pos.x - 100.0, 1.0, pos.z + 60.0));

    let healthbar_width = 5.0;
    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(healthbar_width, 1.0)));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(
        healthbar_width,
        Vec3::new(0.0, 4.5, 0.0),
        healthbar_img,
        healthbar_mesh,
    );

    cmds.spawn(soldier).with_children(|parent| {
        parent.spawn(healthbar);
    });
    println!("Building Unit");
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
            Interaction::Pressed => cmds.trigger(PurchaseSoldierRequestEv::new(50)),
            _ => (),
        }
    }
}
