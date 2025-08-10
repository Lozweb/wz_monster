use bevy::asset::Assets;
use bevy::image::TextureAtlasLayout;
use bevy::log::error;
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Commands, Entity, Mesh, Query, Res, ResMut, Transform};
use bevy_rapier2d::geometry::Group;
use bevy_renet2::prelude::{ClientId, RenetServer};
use game_core::network::network::{ServerChannel, ServerMessages};
use game_core::player::command::{spawn_player_entity, SpawnPlayerParams};
use game_core::player::component::{player_physics, spawn_player_sensor, PlayerNetwork};
use game_core::player::texture::{PlayerTextureType, PlayerTextures};
use game_core::weapon::command::spawn_weapon_entity;
use game_core::weapon::texture::{WeaponTextureType, WeaponTextures};

#[allow(clippy::too_many_arguments)]
pub fn create_player(
    position: Vec3,
    client_id: &ClientId,
    player_texture_type: &PlayerTextureType,
    weapon_texture_type: &WeaponTextureType,
    commands: &mut Commands,
    player_textures: &mut Res<PlayerTextures>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    weapon_textures: &mut Res<WeaponTextures>,
) -> Entity {
    let (pivot, weapon) = spawn_weapon_entity(
        commands,
        meshes,
        materials,
        texture_atlas_layouts,
        weapon_textures,
        weapon_texture_type,
    );
    let sensor = spawn_player_sensor(commands);

    let player_args = SpawnPlayerParams {
        pivot,
        weapon,
        sensor: Some(sensor),
        position,
        player_texture_type,
        client_id: *client_id,
    };

    let player_entity = spawn_player_entity(commands, texture_atlas_layouts, player_textures, player_args);

    commands.entity(player_entity).insert(player_physics());
    commands.entity(player_entity).add_child(sensor);
    commands.entity(player_entity).add_child(pivot);
    commands.entity(pivot).add_child(weapon);
    player_entity
}

pub fn broadcast_player_create(
    player_entity: Entity,
    client_id: &ClientId,
    position: Vec3,
    player_texture_entity_type: PlayerTextureType,
    weapon_texture_entity_type: WeaponTextureType,
    server: &mut ResMut<RenetServer>,
) {
    let message = match bincode::serialize(&ServerMessages::PlayerCreate {
        id: *client_id,
        entity: player_entity.to_bits(),
        translation: position.into(),
        player_texture_entity_type,
        weapon_texture_entity_type,
    }) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Erreur de sérialisation PlayerCreate: {:?}", e);
            return;
        }
    };

    server.broadcast_message(ServerChannel::ServerMessages, message);
}

pub fn send_existing_players_to_client(
    player_texture_entity_type: PlayerTextureType,
    weapon_texture_entity_type: WeaponTextureType,
    client_id: &ClientId,
    players: &mut Query<(Entity, &PlayerNetwork, &Transform)>,
    server: &mut ResMut<RenetServer>,
) {
    for (entity, player, transform) in players.iter() {
        let translation: [f32; 3] = transform.translation.into();

        let message = match bincode::serialize(&ServerMessages::PlayerCreate {
            id: player.id,
            entity: entity.to_bits(),
            translation,
            player_texture_entity_type: player_texture_entity_type.clone(),
            weapon_texture_entity_type: weapon_texture_entity_type.clone(),
        }) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Erreur de sérialisation PlayerCreate: {:?}", e);
                return;
            }
        };
        server.send_message(*client_id, ServerChannel::ServerMessages, message);
    }
}