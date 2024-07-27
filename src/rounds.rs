use bevy::prelude::*;

use crate::resources::RoundInfo;

pub struct RoundsPlugin;

impl Plugin for RoundsPlugin {
    fn build(&self, app: &mut App) {}
}

fn advance_round(mut cmds: Commands, mut round_info: ResMut<RoundInfo>) {
    // let defeated_enemies =
}
