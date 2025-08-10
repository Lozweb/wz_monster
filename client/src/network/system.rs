use crate::network::{ClientLobby, NetworkMapping, PlayerInfo};
use bevy::asset::Assets;

use bevy::image::TextureAtlasLayout;
use bevy::prelude::{info, ColorMaterial, Commands, Entity, Mesh, Res, ResMut};
use bevy_renet2::prelude::RenetClient;
use game_core::network::network::{ServerChannel, ServerMessages};
use game_core::player::command::{spawn_player_entity, SpawnPlayerParams};
use game_core::player::component::{ControlledPlayer, CurrentClientId};
use game_core::player::texture::{PlayerTextureEntity, PlayerTextureType, PlayerTextures};
use game_core::weapon::command::spawn_weapon_entity;
use game_core::weapon::texture::{WeaponTextureType, WeaponTextures};

#[allow(clippy::too_many_arguments)]
pub fn client_event(
    client_id: Res<CurrentClientId>,
    mut player_textures: Res<PlayerTextures>,
    mut weapon_textures: Res<WeaponTextures>,
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
        let server_message = match bincode::deserialize(&message) {
            Ok(msg) => msg,
            Err(e) => {
                info!("Erreur de désérialisation du message serveur: {:?}", e);
                continue;
            }
        };

        match server_message {
            ServerMessages::PlayerCreate { id, translation, entity, player_texture_entity_type, weapon_texture_entity_type } => {
                info!("Player created: {id} at {translation:?}");

                let client_entity = client_create_player_entity(
                    client_id,
                    translation,
                    player_texture_entity_type,
                    weapon_texture_entity_type,
                    &mut commands,
                    &mut texture_atlas_layouts,
                    &mut player_textures,
                    &mut meshes,
                    &mut materials,
                    &mut weapon_textures,
                );

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

#[allow(clippy::too_many_arguments)]
pub fn client_create_player_entity(
    client_id: u64,
    translation: [f32; 3],
    player_texture_type: PlayerTextureType,
    weapon_texture_type: WeaponTextureType,
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &mut Res<PlayerTextures>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    weapon_textures: &mut Res<WeaponTextures>,
) -> Entity {
    let position = translation.into();
    let rick_texture = PlayerTextureEntity::new(&player_texture_type);

    let (pivot, weapon) = spawn_weapon_entity(
        commands,
        meshes,
        materials,
        texture_atlas_layouts,
        weapon_textures,
        &weapon_texture_type,
    );

    let player_args = SpawnPlayerParams {
        pivot,
        weapon,
        position,
        sensor: None,
        player_texture_type: &rick_texture.player_texture_type,
        client_id,
    };

    let player = spawn_player_entity(
        commands,
        texture_atlas_layouts,
        player_textures,
        player_args,
    );

    commands.entity(player).add_child(pivot);
    commands.entity(pivot).add_child(weapon);
    player
}

