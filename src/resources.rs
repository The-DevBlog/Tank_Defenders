use bevy::{audio::Source, prelude::*};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bank>()
            .init_resource::<MouseCoords>()
            .init_resource::<BoxCoords>()
            .init_resource::<GameCommands>()
            .init_resource::<CustomCursor>()
            .init_resource::<MyAssets>()
            .add_systems(Startup, setup);
    }
}

#[derive(Resource, Default, Debug)]
pub struct MyAssets {
    pub full_health: Handle<Image>,
}

#[derive(Resource, Debug)]
pub struct Bank(pub i32);

impl Default for Bank {
    fn default() -> Self {
        Bank(1500)
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

#[derive(Resource, Default)]
pub struct GameCommands {
    pub drag_select: bool,
    pub single_select: bool,
    pub selected: bool,
}

#[derive(Resource)]
pub struct CustomCursor {
    pub state: CursorState,
}

#[derive(PartialEq)]
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

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

fn setup(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    my_assets.full_health = assets.load("imgs/full_health.png");
}
