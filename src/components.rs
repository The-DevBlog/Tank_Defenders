use bevy::prelude::*;
use bevy_mod_billboard::{
    Billboard, BillboardMeshHandle, BillboardTextureBundle, BillboardTextureHandle,
};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Target(pub Option<Entity>);

#[derive(Component)]
pub struct Friendly;

#[derive(Component)]
pub struct Health(pub i32);

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

#[derive(Bundle)]
pub struct UnitBundle {
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub name: Name,
    pub rigid_body: RigidBody,
    pub speed: Speed,
    pub destination: Destination,
    pub unit: Unit,
    pub target: Target,
    pub locked_axis: LockedAxes,
    pub scene_bundle: SceneBundle,
    pub health: Health,
    pub range: Range,
    pub damage: Damage,
    pub fire_rate: FireRate,
    pub current_action: CurrentAction,
}

impl UnitBundle {
    pub fn new(
        name: String,
        speed: f32,
        damage: i32,
        size: Vec3,
        health: i32,
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
            unit: Unit,
            fire_rate: FireRate(fire_rate),
            range: Range(50.0),
            health: Health(health),
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

#[derive(Bundle)]
pub struct HealthbarBundle {
    pub texture: BillboardTextureBundle,
    pub name: Name,
}

impl HealthbarBundle {
    pub fn new(translation: Vec3, img: Handle<Image>, mesh: Handle<Mesh>) -> Self {
        Self {
            texture: BillboardTextureBundle {
                transform: Transform::from_translation(translation),
                texture: BillboardTextureHandle(img),
                mesh: BillboardMeshHandle(mesh),
                ..default()
            },
            name: Name::new("Healthbar"),
        }
    }
}
