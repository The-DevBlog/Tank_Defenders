use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    resources::{CursorState, CustomCursor, GameCommands, MouseCoords},
    Action, CurrentAction, Damage, Destination, Enemy, FireRate, Friendly, Health, InvokeDamage,
    Range, Selected, Speed, Target, Unit, UpdateBankBalanceEv,
};

pub struct SoldiersPlugin;

impl Plugin for SoldiersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                set_unit_destination,
                move_unit,
                command_attack,
                attack,
                attack_if_in_radius,
            ),
        );
    }
}

pub fn set_unit_destination(
    mouse_coords: ResMut<MouseCoords>,
    mut friendly_q: Query<(&mut Destination, &mut Target, &Transform, &Selected), With<Friendly>>,
    input: Res<ButtonInput<MouseButton>>,
    game_cmds: Res<GameCommands>,
    cursor: Res<CustomCursor>,
) {
    if !input.just_released(MouseButton::Left) || game_cmds.drag_select {
        return;
    }

    for (mut unit_destination, mut target, trans, selected) in friendly_q.iter_mut() {
        if !selected.0 {
            continue;
        }

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
            &mut CurrentAction,
            &mut Transform,
            &mut ExternalImpulse,
            &Speed,
            &mut Destination,
        ),
        With<Unit>,
    >,
    time: Res<Time>,
) {
    for (mut action, mut trans, mut ext_impulse, speed, mut destination) in unit_q.iter_mut() {
        if let Some(new_pos) = destination.0 {
            // Update the unit's rotation to face the direction
            let distance = new_pos - trans.translation;

            // Calculate the direction vector on the XZ plane
            let direction = Vec3::new(distance.x, 0.0, distance.z).normalize();

            // Calculate the target Y rotation (yaw)
            let target_yaw = direction.x.atan2(direction.z); // Corrected yaw calculation
            let target_rotation = Quat::from_rotation_y(target_yaw); // Remove adjustment for facin
            trans.rotation = target_rotation;

            if distance.length_squared() <= 5.0 {
                destination.0 = None;
                action.0 = Action::None;
                // println!("Unit Stopping");
            } else {
                action.0 = Action::Relocate;
                // Set the impulse to move the unit
                ext_impulse.impulse += direction * speed.0 * time.delta_seconds();
            }
        }
    }
}

fn command_attack(
    rapier_context: Res<RapierContext>,
    mut select_q: Query<(&Selected, &mut Target), With<Selected>>,
    enemy_q: Query<Entity, With<Enemy>>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mouse_coords: Res<MouseCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor: ResMut<CustomCursor>,
    game_cmds: Res<GameCommands>,
) {
    if !game_cmds.selected {
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
                for (selected, mut target) in select_q.iter_mut() {
                    if selected.0 {
                        target.0 = Some(enemy_ent);
                    }
                }
            }
            return;
        }
    }

    cursor.state = CursorState::Relocate;
}

fn attack(
    mut cmds: Commands,
    mut unit_q: Query<
        (
            &Damage,
            &Range,
            &Transform,
            &mut Destination,
            &mut Target,
            &mut FireRate,
            &mut CurrentAction,
        ),
        With<Unit>,
    >,
    time: Res<Time>,
    mut health_q: Query<&mut Health>,
    target_transform_q: Query<&Transform>,
) {
    for (dmg, range, transform, mut destination, mut target, mut fire_rate, mut action) in
        unit_q.iter_mut()
    {
        if let Some(target_ent) = target.0 {
            if let Ok(enemy_transform) = target_transform_q.get(target_ent) {
                // only attack when enemy is in range
                let distance = (transform.translation - enemy_transform.translation).length();
                if distance <= range.0 {
                    destination.0 = None;
                    action.0 = Action::Attack;

                    // Rotate to face the target
                    // let direction =
                    //     (enemy_transform.translation - transform.translation).normalize();
                    // transform.rotation = Quat::from_rotation_arc(Vec3::X, direction);

                    if let Ok(mut health) = health_q.get_mut(target_ent) {
                        // Despawn if health < 0
                        if health.current <= 0.0 {
                            action.0 = Action::None;
                            cmds.trigger(UpdateBankBalanceEv::new(100));
                            cmds.entity(target_ent).despawn_recursive();
                        }

                        if fire_rate.0.elapsed().is_zero() {
                            // Trigger the damage event at the start of the timer
                            cmds.trigger(InvokeDamage::new(dmg.0, target_ent));
                            health.current -= dmg.0;
                        }
                    }

                    if fire_rate.0.finished() {
                        fire_rate.0.reset();
                    } else {
                        fire_rate.0.tick(time.delta());
                    }
                } else {
                    destination.0 = Some(enemy_transform.translation);
                }
            } else {
                target.0 = None;
            }
        } else {
            action.0 = Action::None;
        }
    }
}

fn attack_if_in_radius(
    mut friendly_q: Query<(&Range, &Transform, &mut Target, &CurrentAction), With<Friendly>>,
    enemy_q: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (range, friendly_transform, mut target, action) in friendly_q.iter_mut() {
        // if action.0 == Action::Relocate {
        //     return;
        // }

        if action.0 == Action::None && target.0.is_none() {
            let mut found_target = false;
            for (enemy_ent, enemy_transform) in enemy_q.iter() {
                let distance =
                    (friendly_transform.translation - enemy_transform.translation).length();
                if distance <= range.0 {
                    target.0 = Some(enemy_ent);
                    found_target = true;
                    break;
                }
            }
            if !found_target {
                target.0 = None;
            }
        }
    }
}
