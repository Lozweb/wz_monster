use crate::entities::player::component::{AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player};
use bevy::prelude::{ChildOf, EventReader, Query, Res, Sprite, Time, With};
use bevy_rapier2d::prelude::{CollisionEvent, Velocity};

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
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite, &Velocity), With<Player>>,
) {
    for (indices, mut timer, mut sprite, velocity) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last || velocity.linvel.x.abs() < 0.1 {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}