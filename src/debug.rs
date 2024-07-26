use bevy::prelude::*;

use crate::{CurrentAction, Friendly};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_friendly_action);
    }
}

fn print_friendly_action(action_q: Query<&CurrentAction, With<Friendly>>) {
    // for action in action_q.iter() {
    //     println!("Action: {:?}", action.0);
    // }
}
