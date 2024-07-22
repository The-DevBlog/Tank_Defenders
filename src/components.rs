use bevy::prelude::*;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Destination(pub Option<Vec3>);

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct MapBase;
