use std::f32::EPSILON;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    resources::{GameCommands, MouseCoords},
    Destination, HealthbarBundle, Selected, Speed, Unit, UnitBundle,
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_unit)
            .add_systems(Update, (set_unit_destination, move_unit));
    }
}

fn spawn_unit(mut cmds: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let unit_scene = assets.load("soldier.glb#Scene0");
    let unit = UnitBundle::new(
        "Soldier".to_string(),
        5000.0,
        2.0,
        unit_scene,
        Vec3::new(0.0, 1.0, 0.0),
    );

    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        unit.scene_bundle.transform.scale.x * 5.0,
        1.0,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(Vec3::new(0.0, 4.5, 0.0), healthbar_img, healthbar_mesh);

    cmds.spawn(unit).with_children(|parent| {
        parent.spawn(healthbar);
    });
}

pub fn set_unit_destination(
    mouse_coords: ResMut<MouseCoords>,
    mut unit_q: Query<(&mut Destination, &Transform), With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
    game_cmds: Res<GameCommands>,
) {
    if !input.just_released(MouseButton::Left) || game_cmds.drag_select {
        return;
    }

    for (mut unit_destination, trans) in unit_q.iter_mut() {
        let mut destination = mouse_coords.global;
        destination.y += trans.scale.y / 2.0; // calculate for entity height
        unit_destination.0 = Some(destination);
        println!("Unit Moving");
    }
}

fn move_unit(
    mut unit_q: Query<(&Transform, &mut ExternalImpulse, &Speed, &mut Destination), With<Unit>>,
    time: Res<Time>,
) {
    for (trans, mut ext_impulse, speed, mut destination) in unit_q.iter_mut() {
        if let Some(new_pos) = destination.0 {
            let distance = new_pos - trans.translation;
            if distance.length_squared() <= (speed.0 * time.delta_seconds()).powi(2) + EPSILON {
                destination.0 = None;
                println!("Unit Stopping");
            } else {
                ext_impulse.impulse += distance.normalize() * speed.0 * time.delta_seconds();
            }
        }
    }
}
