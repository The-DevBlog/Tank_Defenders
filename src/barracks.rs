use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use crate::{Barracks, BuySoldierBtn, HealthbarBundle, PurchaseUnitRequestEv};

pub struct BarracksPlugin;

impl Plugin for BarracksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_barracks)
            .add_systems(Update, buy_soldier_click);
    }
}

fn spawn_barracks(mut cmds: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let barracks = (
        SceneBundle {
            scene: assets.load("Barracks.glb#Scene0"),
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
        Name::new("Barracks"),
    );

    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        barracks.0.transform.scale.x * 45.0,
        2.5,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(Vec3::new(0.0, 22.5, 0.0), healthbar_img, healthbar_mesh);

    cmds.spawn(barracks).with_children(|parent| {
        parent.spawn(healthbar);
    });
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
            Interaction::Pressed => cmds.trigger(PurchaseUnitRequestEv::new(50)),
            _ => (),
        }
    }
}
