// use std::time::Duration;

use bevy::{animation::animate_targets, asset, prelude::*};

// use crate::{resources::Animations, Action, CurrentAction, Friendly};

pub struct AnimationControllerPlugin;

impl Plugin for AnimationControllerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup).add_systems(
        //     Update,
        //     (setup_scene_once_loaded.before(animate_targets), controller),
        // );
        // app.add_systems(Update, controller);
    }
}

// fn setup(mut cmds: Commands, assets: Res<AssetServer>, mut graphs: ResMut<Assets<AnimationGraph>>) {
//     let mut graph = AnimationGraph::new();
//     let animations = graph
//         .add_clips(
//             [GltfAssetLabel::Animation(0).from_asset("soldier_animations.glb")]
//                 .into_iter()
//                 .map(|path| assets.load(path)),
//             1.0,
//             graph.root,
//         )
//         .collect();

//     let graph = graphs.add(graph);
//     cmds.insert_resource(Animations {
//         animations,
//         graph: graph.clone(),
//     });
// }

// // fn setup_scene_once_loaded() {}

// // // Build the animation graph
// // let mut graph = AnimationGraph::new();
// // let animations = graph
// //     .add_clips(
// //         [
// //             GltfAssetLabel::Animation(2).from_asset("models/animated/Fox.glb"),
// //             GltfAssetLabel::Animation(1).from_asset("models/animated/Fox.glb"),
// //             GltfAssetLabel::Animation(0).from_asset("models/animated/Fox.glb"),
// //         ]
// //         .into_iter()
// //         .map(|path| asset_server.load(path)),
// //         1.0,
// //         graph.root,
// //     )
// //     .collect();

// // // Insert a resource with the current scene information
// // let graph = graphs.add(graph);
// // commands.insert_resource(Animations {
// //     animations,
// //     graph: graph.clone(),
// // });

// fn setup_scene_once_loaded(
//     mut commands: Commands,
//     animations: Res<Animations>,
//     mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
// ) {
//     // for (entity, mut player) in &mut players {
//     //     let mut transitions = AnimationTransitions::new();

//     //     // Make sure to start the animation via the `AnimationTransitions`
//     //     // component. The `AnimationTransitions` component wants to manage all
//     //     // the animations and will get confused if the animations are started
//     //     // directly via the `AnimationPlayer`.
//     //     // transitions
//     //     //     .play(&mut player, animations.animations[0], Duration::ZERO)
//     //     //     .repeat();
//     //     transitions.play(&mut player, animations.animations[0], Duration::ZERO);
//     //     commands
//     //         .entity(entity)
//     //         .insert(animations.graph.clone())
//     //         .insert(transitions);
//     // }
// }

// fn controller(
//     mut friendly_q: Query<(Entity, &mut CurrentAction)>,
//     mut animation_players: Query<(Entity, &mut AnimationPlayer)>,
//     mut cmds: Commands,
// ) {
//     for (friendly_ent, current_action) in friendly_q.iter() {
//         // println!("STATE: {:?}", current_action.0);
//         for (animation_ent, mut player) in &mut animation_players {
//             let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
//                 continue;
//             };

//             // let animation = cmds.entity(animation_ent).id();
//             // let friendly = cmds.entity(friendly_ent).id();

//             // if animation == friendly {
//             match current_action.0 {
//                 Action::Attack => {
//                     println!("shooting");
//                     let shooting_animation = player.animation_mut(playing_animation_index).unwrap();
//                     shooting_animation.repeat();
//                 }
//                 Action::Relocate => (),
//                 Action::None => (),
//             }
//             // }
//         }
//     }
// }

// // fn controller(
// //     friendly_query: Query<(Entity, &CurrentAction), With<Friendly>>,
// //     mut animation_players: Query<(Entity, &mut AnimationPlayer, &mut AnimationTransitions)>,
// // ) {
// //     for (friendly_entity, current_action) in friendly_query.iter() {
// //         println!("STATE: {:?}", current_action.0);

// //         if let Ok((entity, mut player, mut transitions)) =
// //             animation_players.get_mut(friendly_entity)
// //         {
// //             let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
// //                 continue;
// //             };

// //             match current_action.0 {
// //                 Action::Attack => {
// //                     let shooting_animation = player.animation_mut(playing_animation_index).unwrap();
// //                     shooting_animation.repeat();
// //                 }
// //                 Action::Relocate => (),
// //                 Action::None => (),
// //             }
// //         }
// //     }
// // }

// #[derive(Component, Deref)]
// pub struct AnimationEntityLink(Entity);

// fn link_animations(
//     mut player_query: Query<(Entity, &Name, &mut AnimationPlayer), Added<AnimationPlayer>>,
//     parent_query: Query<&Parent>,
//     mut animation_links: Query<&mut AnimationLinks>,
//     // assets: Res<MyAssets>,
//     clips: Res<Assets<AnimationClip>>,
// ) {
//     // let Some(action_one) = clips.get(&assets.action_one) else {
//     //     return;
//     // };

//     // let Some(action_two) = clips.get(&assets.action_two) else {
//     //     return;
//     // };

//     // Get all the Animation players which can be deep and hidden in the heirachy
//     for (entity, name, mut player) in player_query.iter_mut() {
//         let top_entity = get_top_parent(entity, &parent_query);
//         let clip_handle: Handle<AnimationClip> = if action_one.compatible_with(name) {
//             // assets.action_one.clone_weak()
//         } else if action_two.compatible_with(name) {
//             // assets.action_two.clone_weak()
//         } else {
//             continue;
//         };

//         // If the top parent has an autobagger component then link player to the parent
//         if let Ok(mut links) = animation_links.get_mut(top_entity) {
//             info!("linking animation to {top_entity:?} for {entity:?}");
//             links.push(AnimationEntityLink(entity.clone()));
//             player.play(clip_handle).pause();
//         }
//     }
// }

// #[derive(Component, Deref, DerefMut, Default)]
// struct AnimationLinks(Vec<AnimationEntityLink>);

// // #[derive(AssetCollection, Resource, Default)]
// // pub struct MyAssets {
// //     #[asset(path = "model.glb#Scene0")]
// //     scene: Handle<Scene>,
// //     #[asset(path = "model.glb#Animation0")]
// //     action_one: Handle<AnimationClip>,
// //     #[asset(path = "model.glb#Animation1")]
// //     action_two: Handle<AnimationClip>,
// // }

// fn animate(
//     time: Res<Time>,
//     mut animation_links: Query<(&AnimationLinks)>,
//     mut players: Query<&mut AnimationPlayer>,
// ) {
//     for (links) in animation_links.iter_mut() {
//         // timer.tick(time.delta());

//         // if timer.finished() {
//         info!("starting animation");
//         for link in links.iter() {
//             if let Ok(mut player) = players.get_mut(link.0) {
//                 player.replay();
//                 player.resume();
//             }
//         }
//         // }
//     }
// }
