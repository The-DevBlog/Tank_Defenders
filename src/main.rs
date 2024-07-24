mod barracks;
mod camera;
mod components;
mod events;
mod hud;
mod map;
mod resources;
mod soldiers;
mod tanks;
mod utils;

use std::time::Duration;

use barracks::BarracksPlugin;
use camera::CameraPlugin;
use components::*;
use events::*;
use hud::HudPlugin;
use map::MapPlugin;
use resources::{Animations, ResourcesPlugin};
use soldiers::SoldiersPlugin;
use tanks::TanksPlugin;
use utils::UtilsPlugin;

use bevy::{animation::animate_targets, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

const MAP_SIZE: f32 = 400.0;

fn main() {
    App::new()
        .add_event::<PurchaseSoldierRequestEv>()
        .add_plugins((
            EventsPlugin,
            DefaultPlugins,
            CameraPlugin,
            MapPlugin,
            HudPlugin,
            BillboardPlugin,
            BarracksPlugin,
            TanksPlugin,
            ResourcesPlugin,
            SoldiersPlugin,
            UtilsPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Update, setup.before(animate_targets))
        .run();
}

fn setup(
    mut cmds: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (ent, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();
        transitions.play(&mut player, animations.animations[0], Duration::ZERO);

        cmds.entity(ent)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}
