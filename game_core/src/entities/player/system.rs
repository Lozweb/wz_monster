use crate::entities::player::component::{AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player, PlayerInput, PlayerTexture};
use bevy::asset::AssetServer;
use bevy::prelude::{ChildOf, Commands, EventReader, Query, Res, Sprite, Time, With};
use bevy_rapier2d::prelude::CollisionEvent;

//todo move to server side
pub fn update_grounded_system(
    mut collision_events: EventReader<CollisionEvent>,
    child_of: Query<&ChildOf>,
    mut grounded_query: Query<(&mut Grounded, &mut JumpCounter)>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) | CollisionEvent::Stopped(e1, e2, _) => {
                let flag = matches!(event, CollisionEvent::Started(_, _, _));
                for entity in [e1, e2] {
                    if let Ok(child) = child_of.get(*entity) {
                        if let Ok((mut grounded, mut jump_counter)) = grounded_query.get_mut(child.parent()) {
                            grounded.0 = flag;
                            if flag {
                                jump_counter.reset();
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite, &PlayerInput), With<Player>>,
) {
    for (indices, mut timer, mut sprite, input) in &mut query {
        if input.left || input.right {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index >= indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
        } else if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = indices.first;
        }

        if input.left {
            sprite.flip_x = false;
        } else if input.right {
            sprite.flip_x = true;
        }
    }
}

pub fn setup_player_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("textures/player1.png");
    commands.insert_resource(PlayerTexture(texture));
}