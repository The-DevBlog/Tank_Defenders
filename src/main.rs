mod animation_controller;
mod audio_controller;
mod barracks;
mod camera;
mod components;
mod debug;
mod events;
mod hud;
mod map;
mod mouse;
mod resources;
mod soldiers;
mod tanks;
mod utils;

use animation_controller::AnimationControllerPlugin;
use audio_controller::AudioControllerPlugin;
use barracks::BarracksPlugin;
use camera::CameraPlugin;
use components::*;
use debug::DebugPlugin;
use events::*;
use hud::HudPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use soldiers::SoldiersPlugin;
use tanks::TanksPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

const MAP_SIZE: f32 = 400.0;
const SPEED_QUANTIFIER: f32 = 1000.0;

fn main() {
    App::new()
        .add_plugins((AnimationControllerPlugin, DebugPlugin))
        .add_plugins((
            EventsPlugin,
            DefaultPlugins,
            AudioControllerPlugin,
            CameraPlugin,
            MapPlugin,
            HudPlugin,
            BillboardPlugin,
            BarracksPlugin,
            ResourcesPlugin,
            TanksPlugin,
            SoldiersPlugin,
            MousePlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
