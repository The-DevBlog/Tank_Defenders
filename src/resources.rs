use std::time::Duration;

use bevy::prelude::*;

use crate::STARTING_FUNDS;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bank>()
            .init_resource::<MouseCoords>()
            .init_resource::<BoxCoords>()
            .init_resource::<GameCommands>()
            .init_resource::<CustomCursor>()
            .init_resource::<MyAssets>()
            .init_resource::<GameInfo>()
            .add_systems(Startup, setup);
    }
}

#[derive(Resource, Default)]
pub struct MyAssets {
    pub img_full_health: Handle<Image>,
    pub img_select_border: Handle<Image>,
    pub img_hud_btn: Handle<Image>,
    pub audio_tank_fire: Handle<AudioSource>,
    pub audio_rifle_fire: Handle<AudioSource>,
    pub audio_unit_select: Vec<Handle<AudioSource>>,
    pub audio_unit_move: Vec<Handle<AudioSource>>,
    pub audio_unit_attack: Vec<Handle<AudioSource>>,
}

#[derive(Resource, Debug)]
pub struct Bank(pub i32);

impl Default for Bank {
    fn default() -> Self {
        Bank(STARTING_FUNDS)
    }
}

impl Bank {
    pub fn reset(&mut self) {
        self.0 = STARTING_FUNDS;
    }
}

#[derive(Resource)]
pub struct GameInfo {
    pub round: i32,
    pub enemy_tanks: i32,
    pub enemy_soldiers: i32,
    pub enemies_killed_round: i32,
    pub enemies_killed_total: i32,
    pub ready_up: bool,
    pub count_down: Timer,
    pub game_over: bool,
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            round: 1,
            enemy_tanks: 0,
            enemy_soldiers: 0,
            enemies_killed_round: 0,
            enemies_killed_total: 0,
            ready_up: false,
            count_down: Timer::from_seconds(5.0, TimerMode::Once),
            game_over: false,
        }
    }
}

impl GameInfo {
    pub fn new_round(&mut self) {
        self.round += 1;
        self.enemies_killed_round = 0;
        self.ready_up = false;
        self.enemy_tanks += 2;
        self.enemy_soldiers += 3;
        self.count_down.reset();
    }

    pub fn restart(&mut self) {
        self.ready_up = false;
        self.enemies_killed_round = 0;
        self.enemies_killed_total = 0;
        self.round = 1;
        self.enemy_tanks = 0;
        self.enemy_soldiers = 0;
        self.count_down.reset();
        self.game_over = false;
    }
}

#[derive(Resource, Default, Debug)]
pub struct MouseCoords {
    pub global: Vec3,
    pub local: Vec2,
}

#[derive(Resource, Default, Debug)]
pub struct BoxCoords {
    pub global_start: Vec3,
    pub global_end: Vec3,
    pub local_start: Vec2,
    pub local_end: Vec2,
}

impl BoxCoords {
    // pub fn empty_local(&mut self) {
    //     self.local_start = Vec2::ZERO;
    //     self.local_end = Vec2::ZERO;
    // }

    pub fn empty_global(&mut self) {
        self.global_start = Vec3::ZERO;
        self.global_end = Vec3::ZERO;
    }
}

#[derive(Resource, Default, Debug)]
pub struct GameCommands {
    pub drag_select: bool,
    pub single_select: bool,
    pub selected: bool,
}

#[derive(Resource)]
pub struct CustomCursor {
    pub state: CursorState,
}

#[derive(PartialEq, Debug)]
pub enum CursorState {
    Attack,
    Relocate,
    Normal,
}

impl Default for CustomCursor {
    fn default() -> Self {
        CustomCursor {
            state: CursorState::Normal,
        }
    }
}

fn setup(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    // TEXTURES
    my_assets.img_select_border = assets.load("imgs/select_border.png");
    my_assets.img_full_health = assets.load("imgs/full_health.png");
    my_assets.img_hud_btn = assets.load("imgs/hud_button.png");
}
