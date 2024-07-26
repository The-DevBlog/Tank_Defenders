use bevy::prelude::*;

use crate::{CurrentAction, Friendly, Target};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_friendly_info);
    }
}

fn print_friendly_info(friendly_q: Query<(&CurrentAction, &Target), With<Friendly>>) {
    for (_action, _target) in friendly_q.iter() {
        // println!("Action: {:?}", action.0);
        // println!("Target: {:?}", target.0);
    }
}
