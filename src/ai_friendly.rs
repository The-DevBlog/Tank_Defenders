use bevy::prelude::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

use crate::{
    resources::{CursorState, CustomCursor, GameCommands, MouseCoords},
    Action, AudioQueuesEv, CurrentAction, Enemy, Friendly, Range, Selected, Target, UnitAudio,
};

pub struct AiFriendlyPlugin;

impl Plugin for AiFriendlyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack_if_in_radius, command_attack));
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

fn command_attack(
    rapier_context: Res<RapierContext>,
    mut select_q: Query<(&Selected, &mut Target), With<Selected>>,
    enemy_q: Query<Entity, With<Enemy>>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mouse_coords: Res<MouseCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor: ResMut<CustomCursor>,
    game_cmds: Res<GameCommands>,
    mut cmds: Commands,
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
                // println!("ATTACK");
                cmds.trigger(AudioQueuesEv(UnitAudio::Attack));
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
