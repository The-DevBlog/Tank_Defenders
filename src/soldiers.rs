use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    resources::{Animations, CursorState, CustomCursor, GameCommands, MouseCoords},
    Damage, Destination, Enemy, FireRate, Friendly, Health, HealthbarBundle, Range, Selected,
    Speed, Target, Unit, UnitBundle,
};

pub struct SoldiersPlugin;

impl Plugin for SoldiersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_soldier).add_systems(
            Update,
            (set_unit_destination, move_unit, command_attack, attack),
        );
    }
}

fn spawn_soldier(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [GltfAssetLabel::Animation(0).from_asset("soldier_animations.glb")]
                .into_iter()
                .map(|path| assets.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    let graph = graphs.add(graph);
    cmds.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });

    let soldier_scene = assets.load("soldier_animations.glb#Scene0");
    let soldier = (
        UnitBundle::new(
            "Soldier".to_string(),
            5000.,
            1,
            Vec3::new(2., 2., 2.),
            50,
            Timer::from_seconds(0.25, TimerMode::Repeating),
            soldier_scene,
            Vec3::new(0.0, 1., 0.0),
        ),
        Friendly,
    );

    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        soldier.0.scene_bundle.transform.scale.x * 5.0,
        1.0,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(Vec3::new(0.0, 4.5, 0.0), healthbar_img, healthbar_mesh);

    cmds.spawn(soldier).with_children(|parent| {
        parent.spawn(healthbar);
    });
}

pub fn set_unit_destination(
    mouse_coords: ResMut<MouseCoords>,
    mut friendly_q: Query<(&mut Destination, &mut Target, &Transform), With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
    game_cmds: Res<GameCommands>,
    cursor: Res<CustomCursor>,
) {
    if !input.just_released(MouseButton::Left) || game_cmds.drag_select {
        return;
    }

    for (mut unit_destination, mut target, trans) in friendly_q.iter_mut() {
        if cursor.state == CursorState::Relocate {
            target.0 = None;
        }

        let mut destination = mouse_coords.global;
        destination.y += trans.scale.y / 2.0; // calculate for entity height
        unit_destination.0 = Some(destination);
        println!("Unit Moving to ({}, {})", destination.x, destination.y);
    }
}

fn move_unit(
    mut unit_q: Query<
        (
            &mut Transform,
            &mut ExternalImpulse,
            &Speed,
            &mut Destination,
        ),
        With<Unit>,
    >,
    time: Res<Time>,
) {
    for (mut trans, mut ext_impulse, speed, mut destination) in unit_q.iter_mut() {
        if let Some(new_pos) = destination.0 {
            let distance = new_pos - trans.translation;
            if distance.length_squared() <= 5.0 {
                destination.0 = None;
                // println!("Unit Stopping");
            } else {
                // Calculate the direction vector on the XZ plane
                let direction = Vec3::new(distance.x, 0.0, distance.z).normalize();

                // Set the impulse to move the unit
                ext_impulse.impulse += direction * speed.0 * time.delta_seconds();

                // Calculate the target Y rotation (yaw)
                let target_yaw = direction.x.atan2(direction.z); // Corrected yaw calculation
                let target_rotation = Quat::from_rotation_y(target_yaw); // Remove adjustment for facing direction

                // Update the unit's rotation to face the direction
                trans.rotation = target_rotation;
            }
        }
    }
}

fn command_attack(
    rapier_context: Res<RapierContext>,
    select_q: Query<Entity, With<Selected>>,
    enemy_q: Query<Entity, With<Enemy>>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mouse_coords: Res<MouseCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor: ResMut<CustomCursor>,
    mut cmds: Commands,
) {
    if select_q.is_empty() {
        cursor.state = CursorState::Normal;
        return;
    }

    let (cam, cam_trans) = cam_q.single();

    let Some(ray) = cam.viewport_to_world(cam_trans, mouse_coords.local) else {
        return;
    };

    let hit = rapier_context.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::only_dynamic(),
    );

    if let Some((enemy_ent, _)) = hit {
        // if ray is cast onto enemy
        if enemy_q.get(enemy_ent).is_ok() {
            cursor.state = CursorState::Attack;

            // if enemy is clicked, command friendlies to attack
            if input.just_pressed(MouseButton::Left) {
                for friendly_ent in select_q.iter() {
                    cmds.entity(friendly_ent).insert(Target(Some(enemy_ent)));
                }
            }
            return;
        }
    }

    cursor.state = CursorState::Relocate;
}

fn attack(
    mut cmds: Commands,
    mut friendly_q: Query<
        (
            &Damage,
            &Range,
            &Transform,
            &mut Destination,
            &mut Target,
            &mut FireRate,
        ),
        With<Friendly>,
    >,
    time: Res<Time>,
    mut health_q: Query<&mut Health>,
    enemy_transform_q: Query<&Transform>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for (damage, range, transform, mut destination, target, mut fire_rate) in friendly_q.iter_mut()
    {
        if let Some(target_ent) = target.0 {
            let Ok(enemy_transform) = enemy_transform_q.get(target_ent) else {
                return;
            };

            // only attack when enemy is in range
            let distance = (transform.translation - enemy_transform.translation).length();
            if distance <= range.0 {
                destination.0 = None;
                fire_rate.0.tick(time.delta());

                //  play animation
                for (mut player, mut transitions) in &mut animation_players {
                    let Some((&playing_animation_index, _)) = player.playing_animations().next()
                    else {
                        continue;
                    };

                    let shooting_animation = player.animation_mut(playing_animation_index).unwrap();
                    shooting_animation.repeat();
                }

                if !fire_rate.0.finished() {
                    return;
                }

                if let Ok(mut health) = health_q.get_mut(target_ent) {
                    println!("Tank Health: {}", health.0);
                    health.0 -= damage.0;

                    // despawn tank if health < 0
                    if health.0 < 0 {
                        cmds.entity(target_ent).despawn_recursive();
                    }
                }
            }
        }
    }
}
