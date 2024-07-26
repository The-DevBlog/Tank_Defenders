use crate::{Action, CurrentAction, Unit};
use bevy::prelude::*;

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, controller);
    }
}

fn controller(unit_q: Query<(&CurrentAction, &AudioSink), With<Unit>>) {
    for (action, sink) in unit_q.iter() {
        match action.0 {
            Action::Relocate => sink.pause(),
            Action::Attack => sink.play(),
            Action::None => sink.pause(),
        }
    }
}
