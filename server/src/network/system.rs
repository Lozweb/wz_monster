use crate::network::player::{broadcast_player_create, create_player, send_existing_players_to_client};
use crate::plugin::ServerLobby;
use bevy::asset::Assets;
use bevy::image::TextureAtlasLayout;
use bevy::log::{error, info};
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Commands, Entity, EventReader, Mesh, Query, Res, ResMut, Sprite, Transform, With};
use bevy_renet2::prelude::{ClientId, RenetServer, ServerEvent};
use game_core::network::network::{ClientChannel, NetworkedEntities, ServerChannel, ServerMessages};
use game_core::player::command::rand_player_texture_entity_type;
use game_core::player::component::{PlayerInput, PlayerNetwork, PlayerWeaponSelected};
use game_core::player::texture::{PlayerTextureType, PlayerTextures};
use game_core::weapon::fx_texture::{FxComponent, WeaponFxTextureType};
use game_core::weapon::texture::WeaponTextures;

#[allow(clippy::too_many_arguments)]
pub fn server_event(
    mut players: Query<(Entity, &PlayerNetwork, &Transform)>,
    mut player_textures: Res<PlayerTextures>,
    mut weapon_textures: Res<WeaponTextures>,
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");

                let position = Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0);
                let player_texture_entity_type = rand_player_texture_entity_type();
                let weapon_texture_entity_type = PlayerWeaponSelected::default_weapon().weapon_texture_type;

                let player_entity = create_player(
                    position,
                    client_id,
                    &player_texture_entity_type,
                    &weapon_texture_entity_type,
                    &mut commands,
                    &mut player_textures,
                    &mut texture_atlas_layouts,
                    &mut meshes,
                    &mut materials,
                    &mut weapon_textures,
                );

                lobby.players.insert(*client_id, player_entity);

                send_existing_players_to_client(
                    player_texture_entity_type.clone(),
                    weapon_texture_entity_type.clone(),
                    client_id,
                    &mut players,
                    &mut server,
                );

                broadcast_player_create(
                    player_entity,
                    client_id,
                    position,
                    player_texture_entity_type,
                    weapon_texture_entity_type,
                    &mut server,
                )
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected {reason:?}");
                if let Some(entity) = lobby.players.remove(client_id) {
                    commands.entity(entity).despawn();
                }

                let message = match bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id }) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Erreur de sérialisation PlayerRemove: {:?}", e);
                        return;
                    }
                };

                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }
}

pub fn handle_players_input(
    mut server: ResMut<RenetServer>,
    mut query: Query<(&PlayerNetwork, &mut PlayerInput)>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            if let Ok(input) = bincode::deserialize::<PlayerInput>(&message) {
                update_player_input(client_id, input, &mut query);
            } else {
                error!("Erreur de désérialisation de PlayerInput pour le client {:?}", client_id);
            }
        }
    }
}

fn update_player_input(
    client_id: ClientId,
    input: PlayerInput,
    players: &mut Query<(&PlayerNetwork, &mut PlayerInput)>,
) {
    for (player_net, mut player_input) in players {
        if player_net.id == client_id {
            *player_input = input;
        }
    }
}

#[allow(clippy::complexity)]
pub fn server_network_sync(
    mut server: ResMut<RenetServer>,
    player_query: Query<(
        Entity,
        &Transform,
        &Sprite,
        &PlayerTextureType,
        &PlayerWeaponSelected,
        &PlayerInput
    ), With<PlayerNetwork>>,
    projectile_query: Query<(Entity, &Transform, &Sprite, &WeaponFxTextureType), With<FxComponent>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform, sprite, texture_entity_type, player_weapon_selected, player_input) in player_query.iter() {
        networked_entities.entities.push(entity.to_bits());
        networked_entities.translations.push(transform.translation.into());

        if let Some(texture) = &sprite.texture_atlas {
            networked_entities.sprite_index.push(texture.index);
            networked_entities.sprite_flip_x.push(sprite.flip_x);
        }

        networked_entities.player_texture_entity_type.push(texture_entity_type.clone());
        networked_entities.weapon_texture_entity_type.push(player_weapon_selected.clone());
        networked_entities.player_aim_direction.push(player_input.aim_direction);
    }

    for (projectile_entity, transform, sprite, weapon_texture_type) in projectile_query.iter() {
        info!("Projectile entity: {:?}", projectile_entity);
        networked_entities.projectile_entities.push(projectile_entity.to_bits());
        networked_entities.projectile_translations.push(transform.translation.into());

        if let Some(texture) = &sprite.texture_atlas {
            networked_entities.projectile_sprite_index.push(texture.index);
            networked_entities.projectile_sprite_flip_y.push(sprite.flip_y);
        }

        networked_entities.weapon_fx_texture_type = weapon_texture_type.clone();
    }

    if !networked_entities.entities.is_empty() {
        let sync_message = bincode::serialize(&networked_entities).unwrap();
        server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
    }
}

