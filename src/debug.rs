use bevy::prelude::*;

use crate::{
    resources::{CustomCursor, GameCommands},
    CurrentAction, Destination, Friendly, Selected, Target,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_friendly_info);
    }
}

fn print_friendly_info(
    friendly_q: Query<(&CurrentAction, &Target, &Selected, &Destination), With<Friendly>>,
    _custom_cursor: Res<CustomCursor>,
    _game_cmds: Res<GameCommands>,
) {
    for (_action, _target, _selected, _destination) in friendly_q.iter() {
        // println!("Action: {:?}", _action.0);
        // println!("Target: {:?}", _target.0);
        println!(
            "Destination: {:?} || Target: {:?}",
            _destination.0, _target.0
        );
        // println!("Selected: {}", _selected.0);
        // println!("Selected: {}", _selected.0);
    }

    // println!("Game Commands: {:?}", _game_cmds);
    // println!("Cursor State: {:?}", _custom_cursor.state);
}
