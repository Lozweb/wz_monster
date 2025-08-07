use crate::network::{ClientLobby, CurrentClientId, NetworkMapping, PlayerInfo};
use bevy::asset::Assets;

use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{info, Children, ColorMaterial, Commands, Entity, Mesh, Query, Res, ResMut, Sprite, Transform, With};
use bevy_renet2::prelude::RenetClient;
use game_core::entities::player::component::{spawn_player_entity, spawn_weapon_entity, ControlledPlayer, PlayerNetwork};
use game_core::entities::player::player_texture::{player_texture_entity_to_handle, PlayerTextureEntity, PlayerTextureEntityType, PlayerTextures};
use game_core::entities::player::system::{is_face_right, radian_to_degrees, weapon_sprite_flip};
use game_core::entities::player::weapon_texture::{PivotDisk, Weapon, WeaponTextures};
use game_core::network::network_entities::{NetworkedEntities, ServerChannel, ServerMessages};

#[allow(clippy::too_many_arguments)]
pub fn client_event(
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
#[allow(clippy::too_many_arguments)]
pub fn update_player_inputs_from_server(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    network_mapping: ResMut<NetworkMapping>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_textures: Res<PlayerTextures>,
    player_query: Query<&Children, With<PlayerNetwork>>,
    mut disk_query: Query<(&mut Transform, &Children), With<PivotDisk>>,
    mut weapon_query: Query<&mut Sprite, With<Weapon>>,
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

                if let Ok(children) = player_query.get(*entity) {
                    for &child in children.iter() {
                        if let Ok((mut transform, weapon_children)) = disk_query.get_mut(child) {
                            let aim_direction = networked_entities.player_aim_direction[i];
                            transform.rotation = Quat::from_rotation_z(aim_direction);
                            if is_face_right(radian_to_degrees(aim_direction)) {
                                transform.translation.x = -2.5;
                            } else {
                                transform.translation.x = 2.5;
                            }
                            for &weapon_entity in weapon_children.iter() {
                                if let Ok(mut weapon_sprite) = weapon_query.get_mut(weapon_entity) {
                                    weapon_sprite_flip(&mut weapon_sprite, aim_direction);
                                }
                            }
                        }
                    }
                }
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