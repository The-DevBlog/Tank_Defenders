mod barracks;
mod camera;
mod components;
mod hud;
mod map;
mod resources;
mod tanks;
mod units;
mod utils;

use barracks::BarracksPlugin;
use camera::CameraPlugin;
use components::*;
use hud::HudPlugin;
use map::MapPlugin;
use resources::ResourcesPlugin;
use tanks::TanksPlugin;
use units::UnitsPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

const MAP_SIZE: f32 = 400.0;
const UNITS: i32 = 50;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            MapPlugin,
            HudPlugin,
            BillboardPlugin,
            BarracksPlugin,
            TanksPlugin,
            ResourcesPlugin,
            UnitsPlugin,
            UtilsPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
