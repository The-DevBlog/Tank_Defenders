use bevy::prelude::*;

use crate::{
    resources::GameInfo, Action, AttackAudioEv, AttackAudioOptions, Barracks, CurrentAction,
    Damage, Destination, Enemy, EnemySoldier, EnemyTank, FireRate, Friendly, GameOverEv, Health,
    InvokeDamage, Range, TankFactory, Target,
};

pub struct AiEnemyPlugin;

impl Plugin for AiEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tank_attack, soldier_attack, attack_if_in_radius));
    }
}

fn attack_if_in_radius(
    mut enemy_q: Query<(&Range, &Transform, &mut Target), With<Enemy>>,
    friendly_q: Query<(Entity, &Transform), With<Friendly>>,
    barracks_q: Query<Entity, With<Barracks>>,
    tank_factory_q: Query<Entity, With<TankFactory>>,
    game_info: Res<GameInfo>,
    mut cmds: Commands,
) {
    for (range, enemy_transform, mut target) in enemy_q.iter_mut() {
        if let Ok(barracks_ent) = barracks_q.get_single() {
            target.0 = Some(barracks_ent);
        } else {
            if let Ok(tank_factory_ent) = tank_factory_q.get_single() {
                target.0 = Some(tank_factory_ent);
            } else {
                if !game_info.game_over {
                    cmds.trigger(GameOverEv);
                }
            }
        }

        for (friendly_ent, friendly_transform) in friendly_q.iter() {
            let distance = (enemy_transform.translation - friendly_transform.translation).length();
            if distance <= range.0 {
                target.0 = Some(friendly_ent);
            }
        }
    }
}

fn tank_attack(
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
        With<EnemyTank>,
    >,
    time: Res<Time>,
    mut health_q: Query<&mut Health>,
    target_transform_q: Query<&Transform>,
) {
    for (dmg, range, transform, mut destination, mut target, mut fire_rate, mut action) in
        unit_q.iter_mut()
    {
        if let Some(target_ent) = target.0 {
            if let Ok(target_transform) = target_transform_q.get(target_ent) {
                // only attack when enemy is in range
                let distance = (transform.translation - target_transform.translation).length();
                if distance <= range.0 {
                    destination.0 = None;
                    action.0 = Action::Attack;

                    if let Ok(mut health) = health_q.get_mut(target_ent) {
                        // Despawn if health < 0
                        if health.current <= 0.0 {
                            action.0 = Action::None;

                            cmds.entity(target_ent).despawn_recursive();
                            return;
                        }

                        if fire_rate.0.elapsed().is_zero() {
                            // Trigger the damage event at the start of the timer
                            cmds.trigger(AttackAudioEv(AttackAudioOptions::Tank));
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
                    destination.0 = Some(target_transform.translation);
                }
            } else {
                target.0 = None;
            }
        } else {
            action.0 = Action::None;
        }
    }
}

fn soldier_attack(
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
        With<EnemySoldier>,
    >,
    time: Res<Time>,
    mut health_q: Query<&mut Health>,
    target_transform_q: Query<&Transform>,
) {
    for (dmg, range, transform, mut destination, mut target, mut fire_rate, mut action) in
        unit_q.iter_mut()
    {
        if let Some(target_ent) = target.0 {
            if let Ok(target_transform) = target_transform_q.get(target_ent) {
                // only attack when enemy is in range
                let distance = (transform.translation - target_transform.translation).length();
                if distance <= range.0 {
                    destination.0 = None;
                    action.0 = Action::Attack;

                    if let Ok(mut health) = health_q.get_mut(target_ent) {
                        // Despawn if health < 0
                        if health.current <= 0.0 {
                            action.0 = Action::None;

                            cmds.entity(target_ent).despawn_recursive();
                            return;
                        }

                        if fire_rate.0.elapsed().is_zero() {
                            // Trigger the damage event at the start of the timer
                            cmds.trigger(AttackAudioEv(AttackAudioOptions::Soldier));
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
                    destination.0 = Some(target_transform.translation);
                }
            } else {
                target.0 = None;
            }
        } else {
            action.0 = Action::None;
        }
    }
}
