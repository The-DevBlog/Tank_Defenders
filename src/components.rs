use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioInstance};
use bevy_mod_billboard::{BillboardMeshHandle, BillboardTextureBundle, BillboardTextureHandle};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MyAssets {
    pub shoot_audio: Option<Handle<bevy_kira_audio::AudioSource>>,
}

// #[derive(Component)]
// pub struct AudioTest(pub Option<Handle<AudioInstance>>);

#[derive(Component)]
pub struct AudioTest(pub Option<AudioChannel>);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct Target(pub Option<Entity>);

#[derive(Component)]
pub struct Friendly;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub original: f32,
}

#[derive(Component)]
pub struct Destination(pub Option<Vec3>);

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct MapBase;

#[derive(Component)]
pub struct Range(pub f32);

#[derive(Component)]
pub struct BuySoldierBtn;

#[derive(Component)]
pub struct BankBalanceTxt;

#[derive(Component)]
pub struct Barracks;

#[derive(Component)]
pub struct FireRate(pub Timer);

#[derive(Component, Debug)]
pub struct CurrentAction(pub Action);

#[derive(Debug)]
pub enum Action {
    Attack,
    Relocate,
    None,
}

// #[derive(Component)]
// pub struct FireAudio(pub Handle<Source>);

#[derive(Bundle)]
pub struct UnitBundle {
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub name: Name,
    pub rigid_body: RigidBody,
    pub speed: Speed,
    pub destination: Destination,
    pub audio_test: AudioTest,
    pub unit: Unit,
    pub target: Target,
    pub locked_axis: LockedAxes,
    pub scene_bundle: SceneBundle,
    pub health: Health,
    pub assets: MyAssets,
    pub range: Range,
    pub damage: Damage,
    pub fire_rate: FireRate,
    pub current_action: CurrentAction,
}

impl UnitBundle {
    pub fn new(
        name: String,
        speed: f32,
        damage: f32,
        size: Vec3,
        health: f32,
        fire_rate: Timer,
        scene: Handle<Scene>,
        translation: Vec3,
    ) -> Self {
        Self {
            collider: Collider::cuboid(size.x, size.y, size.z),
            damping: Damping {
                linear_damping: 5.0,
                ..default()
            },
            external_impulse: ExternalImpulse::default(),
            name: Name::new(name),
            rigid_body: RigidBody::Dynamic,
            speed: Speed(speed),
            target: Target(None),
            damage: Damage(damage),
            destination: Destination(None),
            assets: MyAssets { shoot_audio: None },
            unit: Unit,
            audio_test: AudioTest(None),
            fire_rate: FireRate(fire_rate),
            range: Range(50.0),
            health: Health {
                current: health,
                original: health,
            },
            current_action: CurrentAction(Action::None),
            locked_axis: (LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z),
            scene_bundle: SceneBundle {
                scene: scene,
                transform: Transform {
                    translation: translation,
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct Healthbar {
    pub width: f32,
    pub y_position: f32,
}

#[derive(Bundle)]
pub struct HealthbarBundle {
    pub texture: BillboardTextureBundle,
    pub name: Name,
    pub healthbar: Healthbar,
}

impl HealthbarBundle {
    pub fn new(width: f32, translation: Vec3, img: Handle<Image>, mesh: Handle<Mesh>) -> Self {
        Self {
            texture: BillboardTextureBundle {
                transform: Transform::from_translation(translation),
                texture: BillboardTextureHandle(img),
                mesh: BillboardMeshHandle(mesh),
                ..default()
            },
            // width: HealthbarWidth(width),
            healthbar: Healthbar {
                width: width,
                y_position: translation.y,
            },
            name: Name::new("Healthbar"),
        }
    }
}
