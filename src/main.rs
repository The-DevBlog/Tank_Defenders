mod ai_enemy;
mod animation_controller;
mod audio_controller;
mod barracks;
mod camera;
mod components;
mod debug;
mod events;
mod friendly;
mod hud;
mod map;
mod mouse;
mod resources;
mod rounds;
mod soldiers;
mod tank_factory;
mod tanks;
mod utils;

use ai_enemy::AiEnemyPlugin;
use animation_controller::AnimationControllerPlugin;
use audio_controller::AudioControllerPlugin;
use barracks::BarracksPlugin;
use camera::CameraPlugin;
use components::*;
use debug::DebugPlugin;
use events::*;
use friendly::FriendlyPlugin;
use hud::HudPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use rounds::RoundsPlugin;
use soldiers::SoldiersPlugin;
use tank_factory::TankFactoryPlugin;
use tanks::TanksPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    // render::RapierDebugRenderPlugin,
};

const STARTING_FUNDS: i32 = 2500;
const MAP_SIZE: f32 = 800.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const SOLDIER_DMG: f32 = 20.0;
const SOLDIER_HEALTH: f32 = 250.0;
const SOLDIER_RANGE: f32 = 50.0;
const SOLDIER_SPEED: f32 = 6.5;
const SOLDIER_FIRE_RATE: f32 = 1.5;
const SOLDIER_COST: i32 = 50;
const SOLDIER_REWARD: i32 = 25;
const TANK_DMG: f32 = 80.0;
const TANK_SPEED: f32 = 50.0;
const TANK_FIRE_RATE: f32 = 1.5;
const TANK_COST: i32 = 250;
const TANK_RANGE: f32 = 125.0;
const TANK_HEALTH: f32 = 1000.0;
const TANK_REWARD: i32 = 100;
const TANK_FACTORY_HEALTH: f32 = 3000.0;
const BARRACKS_HEALTH: f32 = 2000.0;

// const MAP_SIZE: f32 = 800.0;
// const SPEED_QUANTIFIER: f32 = 1000.0;
// const SOLDIER_DMG: f32 = 10.0;
// const SOLDIER_HEALTH: f32 = 250.0;
// const SOLDIER_RANGE: f32 = 50.0;
// const SOLDIER_SPEED: f32 = 5.0;
// const SOLDIER_FIRE_RATE: f32 = 1.5;
// const SOLDIER_COST: i32 = 75;
// const SOLDIER_REWARD: i32 = 25;
// const TANK_DMG: f32 = 1000.0;
// const TANK_SPEED: f32 = 150.0;
// const TANK_FIRE_RATE: f32 = 1.5;
// const TANK_COST: i32 = 250;
// const TANK_RANGE: f32 = 125.0;
// const TANK_HEALTH: f32 = 1000.0;
// const TANK_REWARD: i32 = 100;

fn main() {
    App::new()
        .add_plugins((
            RoundsPlugin,
            UtilsPlugin,
            TankFactoryPlugin,
            AnimationControllerPlugin,
            DebugPlugin,
            FriendlyPlugin,
            AiEnemyPlugin,
        ))
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
            // RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            // WorldInspectorPlugin::new(),
        ))
        .run();
}
