use bevy::prelude::*;

use crate::{Barracks, Enemy, Friendly, Range, Target};

pub struct AiEnemyPlugin;

impl Plugin for AiEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_if_in_radius);
    }
}

fn attack_if_in_radius(
    mut enemy_q: Query<(&Range, &Transform, &mut Target), With<Enemy>>,
    friendly_q: Query<(Entity, &Transform), With<Friendly>>,
    barracks_q: Query<Entity, With<Barracks>>,
) {
    for (range, enemy_transform, mut target) in enemy_q.iter_mut() {
        if let Ok(barracks_ent) = barracks_q.get_single() {
            target.0 = Some(barracks_ent);
        }

        for (friendly_ent, friendly_transform) in friendly_q.iter() {
            let distance = (enemy_transform.translation - friendly_transform.translation).length();
            if distance <= range.0 {
                target.0 = Some(friendly_ent);
            }
        }
    }
}
