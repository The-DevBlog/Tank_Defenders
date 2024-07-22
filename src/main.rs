mod barracks;
mod camera;
mod components;
mod events;
mod hud;
mod map;
mod resources;
mod tanks;
mod units;
mod utils;

use barracks::BarracksPlugin;
use camera::CameraPlugin;
use components::*;
use events::*;
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

fn main() {
    App::new()
        .add_event::<PurchaseUnitRequestEv>()
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
            UnitsPlugin,
            UtilsPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
