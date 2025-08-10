use bevy::asset::Assets;
use bevy::image::TextureAtlasLayout;
use bevy::math::Vec3;
use bevy::prelude::{Commands, EventReader, GlobalTransform, Query, Res, ResMut, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::pipeline::CollisionEvent;
use game_core::player::component::{Grounded, JumpCounter, Player, PlayerChildren, PlayerInput, PlayerWeaponSelected};
use game_core::player::math::{apply_jump_velocity, apply_velocity};
use game_core::weapon::command::spawn_weapon_fx;
use game_core::weapon::component::Weapon;
use game_core::weapon::fx_texture::{WeaponFxTextureType, WeaponFxTextures};

pub fn player_move(
    mut query: Query<(
        &Player,
        &PlayerInput,
        &mut Velocity,
        &Grounded,
        &mut JumpCounter
    )>
) {
    for (player, input, mut velocity, grounded, mut jump_counter) in query.iter_mut() {
        apply_velocity(player, input, &mut velocity);
        apply_jump_velocity(player, input, &mut velocity, &mut jump_counter, grounded);
    }
}

pub fn player_jump_control(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Grounded, &mut JumpCounter, &PlayerChildren), With<Player>>,
) {
    for (mut grounded, mut jump_counter, player_children) in player_query.iter_mut() {
        if let Some(sensor) = player_children.sensor {
            for event in collision_events.read() {
                if let CollisionEvent::Started(e1, e2, _) = event {
                    if e1 == &sensor || e2 == &sensor {
                        grounded.0 = true;
                        jump_counter.reset();
                    }
                }
            }
        }
    }
}


pub fn player_shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut weapon_fx_textures: Res<WeaponFxTextures>,
    player_query: Query<(&PlayerInput, &PlayerWeaponSelected, &PlayerChildren), With<Player>>,
    weapon_query: Query<&GlobalTransform, With<Weapon>>,
) {
    for (player_input, player_weapon_selected, children) in player_query.iter() {
        let Ok(global_transform) = weapon_query.get(children.weapon) else { return };

        if player_input.shoot {
            let position = global_transform.transform_point(Vec3::new(52.5, 0.0, 0.0));

            spawn_weapon_fx(
                &mut commands,
                &mut texture_atlas_layouts,
                &mut weapon_fx_textures,
                position,
                &WeaponFxTextureType::from(&player_weapon_selected.weapon_texture_type),
                player_input.aim_direction,
                true,
            );
        }
    }
}