use bevy::prelude::*;

pub struct TanksPlugin;

impl Plugin for TanksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank);
    }
}

fn spawn_tank(mut cmds: Commands, assets: Res<AssetServer>) {
    let tank = (
        SceneBundle {
            scene: assets.load("Tank.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 50.0)),
            ..default()
        },
        Name::new("Tank"),
    );

    cmds.spawn(tank);
}
