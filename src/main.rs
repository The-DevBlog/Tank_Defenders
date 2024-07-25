mod animation_controller;
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

use animation_controller::AnimationControllerPlugin;
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
            AnimationControllerPlugin,
            TanksPlugin,
            ResourcesPlugin,
            SoldiersPlugin,
            UtilsPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
