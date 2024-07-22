use bevy::prelude::*;
use bevy_mod_billboard::{BillboardMeshHandle, BillboardTextureBundle, BillboardTextureHandle};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Commandable;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Destination(pub Option<Vec3>);

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct MapBase;

#[derive(Component)]
pub struct BuySoldierBtn;

#[derive(Component)]
pub struct BankBalanceTxt;

#[derive(Component)]
pub struct Barracks;

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
    pub locked_axis: LockedAxes,
    pub scene_bundle: SceneBundle,
    pub health: Health,
}

impl UnitBundle {
    pub fn new(
        name: String,
        speed: f32,
        size: Vec3,
        health: i32,
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
            destination: Destination(None),
            unit: Unit,
            health: Health(health),
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
