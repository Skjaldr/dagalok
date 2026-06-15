use bevy::{platform::collections::HashMap, prelude::*, scene::SceneInstanceReady};

use crate::{characters::setup_char::{Animated, IsMoving}, gamestate::GameState};

// For animations there are a few things that are needed in order to get and use the animations.  First, a struct that holds: a handle to an animation graph
// and a HashMap with the Name of the animation, and the index(AnimatnionNodeIndex) to store the index information.  This struct will also need to be added to the
// character entity, so for the get_animations() function we'll need a query to target our entity that contains the character mesh.  An AnimationEntityLink will
// need to be created so we can link the mesh with the skeleton.  Both the source and character files are .glb, but the source file contains the skeleton
// with the animation data, and the character mesh contains the skeleton with the mesh.  Both have the exact same bones used to animate the mesh.

#[derive(Eq, PartialEq, Hash)]
pub enum AnimationName {
    Idle,
    Run,
    _SwordShieldStance,
    _SwordShieldAttack,
    _Death,
}

#[derive(Component)]
pub struct AnimationList {
    pub graph_handle: Handle<AnimationGraph>,
    pub index_map: HashMap<AnimationName, AnimationNodeIndex>
}

// #[derive(Component)]
// pub struct AnimationEntityLink(pub Entity);


// In this function we're grabbing the target entity, which is an entity that has the Animated tag, i.e., all characters
// then grabbing the desired animations, passing them to a struct with a Handle<AnimationGraph> and HashMap with animation nodes to store for later use.
// Then inserting the struct onto the entity so we can animate the already spawned mesh.
pub fn get_animations(
    mut commands: Commands,
    target_query: Query<Entity, With<Animated>>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut next_state: ResMut<NextState<GameState>>,

) {
    let Ok(target_entity) = target_query.single() else {
        return;
    };


    let idle_handle: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(0).from_asset("source.glb"));
    let run_handle: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(2).from_asset("source.glb"));
    // let (graphy, indexy) = AnimationGraph::from_clip(idle_handle.clone());

    let mut graph = AnimationGraph::new();
    let idle_index = graph.add_clip(idle_handle, 1.0, graph.root);
    let run_index = graph.add_clip(run_handle, 1.0, graph.root);
    let graph_handle = graphs.add(graph);

    let mut index_map: HashMap<AnimationName, AnimationNodeIndex> = HashMap::new();
    index_map.insert(AnimationName::Idle, idle_index.clone());
    index_map.insert(AnimationName::Run, run_index.clone());

    let animations = AnimationList {
        graph_handle,
        index_map
    };
    commands.entity(target_entity).insert(animations).observe(play_animations);
    next_state.set(GameState::DoneLoading);
}

// this function iterates through all of the bones on the parent entity, finds the child entity that has an animation player
// then inserts an animation graph on the child, and links the child to the parent entity.  Without this, the animation would not
// be able to run on the separate .glb mesh.  This functionality is very useful for a modular design.
// pub fn link_animations(
//     mut commands: Commands,
//     children: Query<&Children>,
//     parent_entity_query: Query<(Entity, &AnimationList), (With<Animated>, Without<AnimationEntityLink>)>,
//     anim_player: Query<&mut AnimationPlayer>,
// ) {
//     // Grab the entities and their ID that contain an animation list.  Must be tagged with animated, but cannot be linked yet
//     for (parent, animation_list) in parent_entity_query.iter() {
//         // iterate through the parent's children entities
//         for child in children.iter_descendants(parent) {
//             // and if we find a child that contains an animation player
//             if anim_player.contains(child) {
//                 // attach an animation graph handle with the animation list's graph handle
//                 commands.entity(child).insert(AnimationGraphHandle(animation_list.graph_handle.clone()));
//                 // and link the child to the parent
//                 commands.entity(parent).insert(AnimationEntityLink(child));
//             }
//         }
//     }
// }


// currently the animation plays once the observer is called.  After working with is_moving, I've determined that maybe the play_animations function doesn't
// see the updates to the IsMoving component because it only plays once the observer is called.  I'm wondering if there is a way to inject the current playing
// animation utilizing a different function that changes the value on the AnimationList.  Basically, I'll have another function that observes the state of the player.
// if the player IS moving then I want the observer to update the current animation.  If the player is not moving then we want to go back to

pub fn play_animations(
    animations_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animation_list: Query<&AnimationList>,
    mut animation_player: Query<&mut AnimationPlayer>,
) {

    if let Ok(anim_list) = animation_list.get(animations_ready.entity) {
        for child in children.iter_descendants(animations_ready.entity) {

            let Some(idle) = anim_list.index_map.get(&AnimationName::Idle) else {
                return;
            };
            if let Ok(mut player) = animation_player.get_mut(child) {
                if !player.is_playing_animation(*idle) {
                    player.play(*idle);
                }
                commands.entity(child).insert(AnimationGraphHandle(anim_list.graph_handle.clone()));
            }
        }
    }
}

pub fn update_animations(
    mut query: Query<(Entity, &AnimationList, &IsMoving), Changed<IsMoving>>,
    children: Query<&Children>,
    mut player_query: Query<&mut AnimationPlayer>,
) {
    for (entity, anim, is_moving) in query.iter_mut() {
        for child in children.iter_descendants(entity) {

            if let Ok(mut player) = player_query.get_mut(child) {
                let Some(run) = anim.index_map.get(&AnimationName::Run) else {
                    return;
                };
                let Some(idle) = anim.index_map.get(&AnimationName::Idle) else {
                    return;
                };

                if is_moving.0 {
                    if !player.is_playing_animation(*run) {
                        player.stop(*idle);
                        player.play(*run).repeat();
                    }

                } else {
                    if !player.is_playing_animation(*idle) {
                        player.stop(*run);
                        player.play(*idle).repeat();
                    }
                }
            }
        }
    }
}
