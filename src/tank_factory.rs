use bevy::{audio::Volume, prelude::*};
use bevy_rapier3d::{
    prelude::{Collider, RigidBody},
    render::ColliderDebugColor,
};

use crate::{
    BuildTankEv, BuyTankBtn, Friendly, Health, HealthbarBundle, PurchaseUnitRequestEv, Selected,
    TankFactory, UnitBundle, UnitType, MAP_SIZE, SPEED_QUANTIFIER, TANK_COST, TANK_DMG,
    TANK_FIRE_RATE, TANK_HEALTH, TANK_RANGE, TANK_SPEED,
};

pub struct TankFactoryPlugin;

impl Plugin for TankFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank_factory)
            .add_systems(Update, buy_tank_click)
            .observe(build_tank);
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

fn build_tank(
    _trigger: Trigger<BuildTankEv>,
    tank_factory_q: Query<&Transform, With<TankFactory>>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cmds: Commands,
) {
    let Ok(barracks_transform) = tank_factory_q.get_single() else {
        return;
    };

    let pos = barracks_transform.translation;
    let mut tank = (
        UnitBundle::new(
            0,
            "Tank".to_string(),
            TANK_SPEED * SPEED_QUANTIFIER,
            TANK_DMG,
            TANK_RANGE,
            TANK_HEALTH,
            Vec3::new(4., 2., 6.),
            assets.load("audio/tank_fire.ogg"),
            Timer::from_seconds(TANK_FIRE_RATE, TimerMode::Repeating),
            assets.load("tank_friendly.glb#Scene0"),
            Vec3::new(pos.x - 30.0, 1.0, pos.z + 20.0),
        ),
        Selected(false),
        ColliderDebugColor(Hsla::new(120.0, 0.22, 0.3, 0.0)),
        Friendly,
    );

    tank.0.audio.settings.volume = Volume::new(0.01);
    tank.0.destination.0 = Some(Vec3::new(pos.x - 100.0, 1.0, pos.z + 60.0));

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

    println!("Building Tank");
}

fn buy_tank_click(
    mut cmds: Commands,
    mut interact_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuyTankBtn>),
    >,
) {
    for (interaction, mut _background_clr) in &mut interact_q {
        match *interaction {
            Interaction::Pressed => {
                cmds.trigger(PurchaseUnitRequestEv::new(TANK_COST, UnitType::Tank))
            }
            _ => (),
        }
    }
}
