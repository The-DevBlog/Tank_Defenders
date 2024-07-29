use bevy::{animation::animate_targets, prelude::*};

use super::{Action, CurrentAction};

pub struct AnimationControllerPlugin;

impl Plugin for AnimationControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (setup_scene_once_loaded.before(animate_targets), controller),
        );
    }
}

fn controller(
    mut player_q: Query<&mut AnimationPlayer>,
    unit_q: Query<(Entity, &CurrentAction)>,
    children_q: Query<&Children>,
    animations: Res<Animations>,
) {
    for (unit_ent, action) in unit_q.iter() {
        for player_ent in children_q.iter_descendants(unit_ent) {
            if let Ok(mut player) = player_q.get_mut(player_ent) {
                match action.0 {
                    Action::Attack => player.play(animations.animations[0]).repeat(),
                    Action::Relocate => player.play(animations.animations[0]).rewind(),
                    Action::None => player.play(animations.animations[0]).rewind(),
                };
            }
        }
    }
}

fn setup_scene_once_loaded(
    mut cmds: Commands,
    animations: Res<Animations>,
    mut players: Query<Entity, Added<AnimationPlayer>>,
) {
    for entity in &mut players {
        let transitions = AnimationTransitions::new();
        cmds.entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

fn setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [GltfAssetLabel::Animation(0).from_asset("soldier_animations.glb")]
                .into_iter()
                .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    cmds.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });
}

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}
