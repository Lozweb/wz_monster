use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Assets, ColorMaterial, Commands, Entity, EventReader, Mesh, Query, Res, ResMut, Resource, Sprite, TextureAtlasLayout, Transform, With};
use bevy_renet2::netcode::{NativeSocket, NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerSetupConfig};
use bevy_renet2::prelude::{ClientId, RenetServer, ServerEvent};
use game_core::entities::player::component::{player_physics_bundle, spawn_player_entity, spawn_player_sensor, spawn_weapon_entity, PlayerInput, PlayerNetwork, PlayerWeaponSelected};
use game_core::entities::player::player_texture::{rand_player_texture_entity_type, PlayerTextureEntityType, PlayerTextures};
use game_core::entities::player::weapon_texture::WeaponTextures;
use game_core::network::network_entities::{connection_config, ClientChannel, NetworkedEntities, ServerChannel, ServerMessages, PROTOCOL_ID};
use renet2_visualizer::RenetServerVisualizer;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}

pub fn add_netcode_network(app: &mut App) {
    app.add_plugins(NetcodeServerPlugin);

    let server = RenetServer::new(connection_config());

    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: std::time::Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerSetupConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        socket_addresses: vec![vec![public_addr]],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, NativeSocket::new(socket).unwrap()).unwrap();
    app.insert_resource(server);
    app.insert_resource(transport);
}

#[allow(clippy::too_many_arguments)]
pub fn server_event(
    players: Query<(Entity, &PlayerNetwork, &Transform)>,
    mut player_textures: Res<PlayerTextures>,
    mut weapon_textures: Res<WeaponTextures>,
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
                visualizer.add_client(*client_id);

                let position = Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0);
                let player_texture_entity_type = rand_player_texture_entity_type();
                let weapon_texture_entity_type = PlayerWeaponSelected::default_weapon().weapon_entity_type;

                for (entity, player, transform) in players.iter() {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity: entity.to_bits(),
                        translation,
                        player_texture_entity_type: player_texture_entity_type.clone(),
                        weapon_texture_entity_type: weapon_texture_entity_type.clone(),
                    }).unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                let player_entity = spawn_player_entity(
                    &mut commands,
                    &mut texture_atlas_layouts,
                    &mut player_textures,
                    position,
                    &player_texture_entity_type,
                    *client_id,
                );

                commands.entity(player_entity).insert(player_physics_bundle());

                let sensor = spawn_player_sensor(&mut commands);

                commands.entity(player_entity).add_child(sensor);

                let (disk_entity, weapon_entity) = spawn_weapon_entity(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut texture_atlas_layouts,
                    &mut weapon_textures,
                    &weapon_texture_entity_type,
                );

                commands.entity(player_entity).add_child(disk_entity);
                commands.entity(disk_entity).add_child(weapon_entity);

                lobby.players.insert(*client_id, player_entity);

                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: *client_id,
                    entity: player_entity.to_bits(),
                    translation: position.into(),
                    player_texture_entity_type,
                    weapon_texture_entity_type,
                }).unwrap();

                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected {reason:?}");
                if let Some(entity) = lobby.players.remove(client_id) {
                    commands.entity(entity).despawn();
                }
                visualizer.remove_client(*client_id);

                let message = bincode::serialize(
                    &ServerMessages::PlayerRemove { id: *client_id }).unwrap();

                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }
}

pub fn update_player_inputs_from_clients(
    mut server: ResMut<RenetServer>,
    mut query: Query<(&PlayerNetwork, &mut PlayerInput)>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            for (player_net, mut player_input) in &mut query {
                if player_net.id == client_id {
                    *player_input = input;
                }
            }
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
        &PlayerTextureEntityType,
        &PlayerWeaponSelected,
        &PlayerInput
    ), With<PlayerNetwork>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform, sprite, texture_entity_type, player_weapon_selected, player_input) in player_query.iter() {
        networked_entities.entities.push(entity.to_bits());
        networked_entities.translations.push(transform.translation.into());
        if let Some(texture) = &sprite.texture_atlas {
            networked_entities.sprite_index.push(texture.index);
            networked_entities.sprite_flip_x.push(sprite.flip_x);
            networked_entities.player_texture_entity_type.push(texture_entity_type.clone());
        }
        networked_entities.weapon_texture_entity_type.push(player_weapon_selected.clone());
        networked_entities.player_aim_direction.push(player_input.aim_direction);
    }

    if !networked_entities.entities.is_empty() {
        let sync_message = bincode::serialize(&networked_entities).unwrap();
        server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
    }
}

