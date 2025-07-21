use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{AssetServer, Assets, Commands, Entity, EventReader, Name, Query, Res, ResMut, Resource, Sprite, TextureAtlas, TextureAtlasLayout, Timer, TimerMode, Transform, UVec2, With};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Friction, GravityScale, LockedAxes, RigidBody, Sensor, Velocity};
use bevy_renet2::netcode::{NativeSocket, NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerSetupConfig};
use bevy_renet2::prelude::{ClientId, RenetServer, ServerEvent};
use game_core::entities::player::component::{AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player, PlayerInput, PlayerNetwork, PLAYER_SPRITE};
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
pub fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    players: Query<(Entity, &PlayerNetwork, &Transform)>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
                visualizer.add_client(*client_id);

                // Spawn existing players for the new client
                for (entity, player, transform) in players.iter() {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity: entity.to_bits(),
                        translation,
                    }).unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                // Spawn a new system
                let position = Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0);
                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(32, 48), 4, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                let animation_indices = AnimationIndices { first: 1, last: 3 };

                let player_entity = commands.spawn((
                    Name::new("Player"),
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Velocity::zero(),
                    Collider::cuboid(16.0, 24.0),
                    GravityScale(1.),
                    Friction::coefficient(0.0),
                    Sprite::from_atlas_image(
                        asset_server.load(PLAYER_SPRITE),
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index: animation_indices.first,
                        },
                    ),
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Player(350.),
                    PlayerInput::default(),
                    Grounded(false),
                    JumpCounter { jumps_left: 2, max_jumps: 2 },
                    Transform::from_translation(position),
                ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("Player Sensor"),
                            Collider::cuboid(10.0, 2.0),
                            Sensor,
                            ActiveEvents::COLLISION_EVENTS,
                            Transform::from_xyz(0.0, -34.0, 0.0),
                        ));
                    })
                    .insert(PlayerNetwork { id: *client_id })
                    .id();


                // Send the new system creation message to all clients
                lobby.players.insert(*client_id, player_entity);

                let translation: [f32; 3] = position.into();
                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: player_entity.to_bits(),
                    entity: player_entity.to_bits(),
                    translation,
                }).unwrap();

                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected {reason:?}");
                if let Some(entity) = lobby.players.remove(client_id) {
                    commands.entity(entity).despawn();
                }
                visualizer.remove_client(*client_id);

                let message = bincode::serialize(&ServerMessages::PlayerRemove {
                    id: *client_id,
                }).unwrap();

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

pub fn server_network_sync(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform, &Sprite), With<PlayerNetwork>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform, sprite) in query.iter() {
        networked_entities.entities.push(entity.to_bits());
        networked_entities.translations.push(transform.translation.into());
        if let Some(texture) = &sprite.texture_atlas {
            networked_entities.sprite_index.push(texture.index);
            networked_entities.sprite_flip_x.push(sprite.flip_x);
        }
    }
    if !networked_entities.entities.is_empty() {
        let sync_message = bincode::serialize(&networked_entities).unwrap();
        server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
    }
}

