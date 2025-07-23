use crate::network::{ClientLobby, CurrentClientId, NetworkMapping, PlayerInfo};
use bevy::asset::Assets;

use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::Vec3;
use bevy::prelude::{info, ColorMaterial, Commands, Entity, Mesh, Res, ResMut, Sprite, Transform};
use bevy_renet2::prelude::RenetClient;
use game_core::entities::player::component::ControlledPlayer;
use game_core::entities::player::entity::spawn_player_entity;
use game_core::entities::player::texture::{player_texture_entity_to_handle, PlayerTextureEntity, PlayerTextureEntityType, PlayerTextures};
use game_core::entities::weapons::entity::spawn_weapon_entity;
use game_core::entities::weapons::texture::WeaponTextures;
use game_core::network::network_entities::{NetworkedEntities, ServerChannel, ServerMessages};

#[allow(clippy::too_many_arguments)]
pub fn client_sync_players(
    mut player_textures: Res<PlayerTextures>,
    mut weapon_textures: Res<WeaponTextures>,
    client_id: Res<CurrentClientId>,
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut network_mapping: ResMut<NetworkMapping>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let client_id = client_id.0;
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            ServerMessages::PlayerCreate { id, translation, entity, player_texture_entity_type, weapon_texture_entity_type } => {
                info!("Player created: {id} at {translation:?}");

                let position = translation.into();
                let rick_texture = PlayerTextureEntity::new(&player_texture_entity_type);

                let client_entity = spawn_player_entity(
                    &mut commands,
                    &mut texture_atlas_layouts,
                    &mut player_textures,
                    position,
                    &rick_texture.player_texture_entity_type,
                    client_id,
                );

                let (disk_entity, weapon_entity) = spawn_weapon_entity(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut texture_atlas_layouts,
                    &mut weapon_textures,
                    &weapon_texture_entity_type,
                );

                commands.entity(client_entity).add_child(disk_entity);
                commands.entity(disk_entity).add_child(weapon_entity);

                if client_id == id {
                    commands.entity(client_entity).insert(ControlledPlayer);
                }

                let player_info = PlayerInfo {
                    server_entity: Entity::from_bits(entity),
                    client_entity,
                };

                lobby.players.insert(id, player_info);
                network_mapping.0.insert(Entity::from_bits(entity), client_entity);
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player removed: {id}");
                if let Some(PlayerInfo {
                                server_entity,
                                client_entity
                            }) = lobby.players.remove(&id) {
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
        }
    }
}
pub fn update_player_inputs_from_server(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    network_mapping: ResMut<NetworkMapping>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_textures: Res<PlayerTextures>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&Entity::from_bits(networked_entities.entities[i])) {
                animate_player(
                    entity,
                    networked_entities.sprite_index[i],
                    networked_entities.sprite_flip_x[i],
                    &networked_entities.player_texture_entity_type[i],
                    networked_entities.translations[i].into(),
                    &mut commands,
                    &mut texture_atlas_layouts,
                    &player_textures,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn animate_player(
    entity: &Entity,
    sprite_index:
    usize, sprite_flip_x:
    bool, texture_entity_type: &PlayerTextureEntityType,
    translation: Vec3,
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &Res<PlayerTextures>,
) {
    let (image, layout) = player_texture_entity_to_handle(
        texture_entity_type,
        texture_atlas_layouts,
        player_textures,
    );

    commands.entity(*entity)
        .insert(Sprite {
            image,
            texture_atlas: Some(TextureAtlas {
                layout,
                index: sprite_index,
            }),
            flip_x: sprite_flip_x,
            ..Default::default()
        })
        .insert(Transform {
            translation,
            scale: Vec3::splat(0.5),
            ..Default::default()
        });
}