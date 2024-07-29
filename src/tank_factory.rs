use bevy::{audio::Volume, prelude::*};
use bevy_mod_billboard::{BillboardTextureBundle, BillboardTextureHandle};
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::{
    resources::MyAssets, BorderSelect, BuildTankEv, BuyTankBtn, Friendly, Health, HealthbarBundle,
    PurchaseUnitRequestEv, RestartGameEv, Selected, Tank, TankFactory, UnitBundle, UnitType,
    MAP_SIZE, SPEED_QUANTIFIER, TANK_COST, TANK_DMG, TANK_FIRE_RATE, TANK_HEALTH, TANK_RANGE,
    TANK_SPEED,
};

pub struct TankFactoryPlugin;

impl Plugin for TankFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buy_tank_click)
            .observe(spawn_tank_factory)
            .observe(build_tank);
    }
}

fn spawn_tank_factory(
    _trigger: Trigger<RestartGameEv>,
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
    my_assets: Res<MyAssets>,
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
            "Tank Friendly".to_string(),
            TANK_SPEED * SPEED_QUANTIFIER,
            TANK_DMG,
            TANK_RANGE,
            TANK_HEALTH,
            Vec3::new(4., 2., 6.),
            my_assets.audio_tank_fire.clone(),
            Timer::from_seconds(TANK_FIRE_RATE, TimerMode::Repeating),
            assets.load("tank_friendly.glb#Scene0"),
            Vec3::new(pos.x - 30.0, 2.0, pos.z + 20.0),
        ),
        Selected(false),
        Friendly,
        Tank,
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

    let select_border = (
        BillboardTextureBundle {
            texture: BillboardTextureHandle(my_assets.img_select_border.clone()),
            ..default()
        },
        BorderSelect::new(15.0, 15.0),
        Name::new("Border Select"),
    );

    cmds.spawn(tank).with_children(|parent| {
        parent.spawn(healthbar);
        parent.spawn(select_border);
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
