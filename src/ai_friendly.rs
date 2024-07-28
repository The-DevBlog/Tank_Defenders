use bevy::prelude::*;

use crate::{Action, CurrentAction, Enemy, Friendly, Range, Target};

pub struct AiFriendlyPlugin;

impl Plugin for AiFriendlyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_if_in_radius);
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
