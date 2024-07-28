use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bank>()
            .init_resource::<MouseCoords>()
            .init_resource::<BoxCoords>()
            .init_resource::<GameCommands>()
            .init_resource::<CustomCursor>()
            .init_resource::<MyAssets>()
            .init_resource::<RoundInfo>()
            .add_systems(Startup, setup);
    }
}

#[derive(Resource, Default)]
pub struct MyAssets {
    pub full_health: Handle<Image>,
    pub select_border: Handle<Image>,
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
        Bank(1500)
    }
}

#[derive(Resource)]
pub struct RoundInfo {
    pub round: i32,
    pub enemy_tanks: i32,
    pub enemy_soldiers: i32,
    pub enemies_defeated: i32,
}

impl Default for RoundInfo {
    fn default() -> Self {
        RoundInfo {
            round: 1,
            enemy_tanks: 0,
            enemy_soldiers: 30,
            enemies_defeated: 0,
        }
    }
}

impl RoundInfo {
    pub fn new_round(&mut self) {
        self.round += 1;
        self.enemies_defeated = 0;
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
    my_assets.select_border = assets.load("imgs/select_border.png");
    my_assets.full_health = assets.load("imgs/full_health.png");
}
