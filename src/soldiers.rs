use bevy::prelude::*;

use crate::{
    Action, AttackAudioEv, AttackAudioOptions, CurrentAction, Damage, Destination, EnemyKilledEv,
    FireRate, Health, InvokeDamage, Range, Reward, Soldier, Target, UpdateBankBalanceEv,
};

pub struct SoldiersPlugin;

impl Plugin for SoldiersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack);
    }
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
        With<Soldier>,
    >,
    time: Res<Time>,
    mut health_q: Query<&mut Health>,
    target_transform_q: Query<&Transform>,
    reward_q: Query<&Reward>,
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

                            if let Ok(reward) = reward_q.get(target_ent) {
                                cmds.trigger(UpdateBankBalanceEv::new(reward.0));
                            }

                            cmds.entity(target_ent).despawn_recursive();
                            cmds.trigger(EnemyKilledEv);
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
