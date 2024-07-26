mod animation_controller;
mod audio_controller;
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

use audio_controller::AudioControllerPlugin;
use barracks::BarracksPlugin;
use camera::CameraPlugin;
use components::*;
use events::*;
use hud::HudPlugin;
use map::MapPlugin;
use resources::ResourcesPlugin;
use soldiers::SoldiersPlugin;
use tanks::TanksPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
const MAP_SIZE: f32 = 400.0;

fn main() {
    App::new()
        // .add_event::<PurchaseSoldierRequestEv>()
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
            UtilsPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
