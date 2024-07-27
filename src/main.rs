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
mod rounds;
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
use rounds::RoundsPlugin;
use soldiers::SoldiersPlugin;
use tanks::TanksPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

// const MAP_SIZE: f32 = 800.0;
// const SPEED_QUANTIFIER: f32 = 1000.0;
// const SOLDIER_DMG: f32 = 5.0;
// const SOLDIER_HEALTH: f32 = 100.0;
// const SOLDIER_RANGE: f32 = 50.0;
// const SOLDIER_SPEED: f32 = 5.0;
// const TANK_DMG: f32 = 50.0;
// const TANK_SPEED: f32 = 22.5;
// const TANK_RANGE: f32 = 125.0;
// const TANK_HEALTH: f32 = 1000.0;
// const TANK_REWARD: i32 = 100;

const MAP_SIZE: f32 = 800.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const SOLDIER_DMG: f32 = 20.0;
const SOLDIER_HEALTH: f32 = 100.0;
const SOLDIER_RANGE: f32 = 50.0;
const SOLDIER_SPEED: f32 = 5.0;
const TANK_DMG: f32 = 1.0;
const TANK_SPEED: f32 = 22.5;
const TANK_RANGE: f32 = 125.0;
const TANK_HEALTH: f32 = 100.0;
const TANK_REWARD: i32 = 100;

fn main() {
    App::new()
        .add_plugins((RoundsPlugin, AnimationControllerPlugin, DebugPlugin))
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
