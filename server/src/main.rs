use bevy::app::{FixedUpdate, Startup};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::Vec3;
use bevy::prelude::{App, Commands, Entity, EventReader, Events, Query, Res, ResMut, Resource, Transform, Update, With};
use bevy::DefaultPlugins;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_renet2::netcode::{NativeSocket, NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerSetupConfig};
use bevy_renet2::prelude::{ClientId, RenetServer, RenetServerPlugin, ServerEvent};
use game_core::entities::decor::system::setup_ground;
use game_core::entities::player::component::{create_player_bundle, PlayerInput};
use game_core::entities::player::system::{animate_sprite, update_grounded_system};
use game_core::network::network_entities::{connection_config, ClientChannel, NetworkedEntities, PlayerNetwork, ServerChannel, ServerMessages, PROTOCOL_ID};
use renet2_visualizer::RenetServerVisualizer;
use server::system::decor_system::setup_camera;
use server::system::player_system::move_player_system;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}


fn add_netcode_network(app: &mut App) {
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
    app.insert_resource(server)
        .insert_resource(transport);
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: false,
    });

    app.insert_resource(ServerLobby::default());
    app.insert_resource(RenetServerVisualizer::<200>::default());

    add_netcode_network(&mut app);

    app.init_resource::<Events<CollisionEvent>>();

    app.add_systems(Update, (
        server_update_system,
        update_visualizer_system,
        animate_sprite
    ));

    app.add_systems(FixedUpdate, (
        update_grounded_system,
        move_player_system,
        server_network_sync,
    ));


    app.add_systems(Startup, (
        setup_camera,
        setup_ground
    ));

    app.run();
}

#[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
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
                let player_entity = commands.spawn(create_player_bundle(client_id, position)).id();

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

    // update system inputs
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(input);
            }
        }
    }
}

fn server_network_sync(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), With<PlayerNetwork>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform) in query.iter() {
        networked_entities.entities.push(entity.to_bits());
        networked_entities.translations.push(transform.translation.into());
    }

    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}


fn update_visualizer_system(mut egui_contexts: EguiContexts, mut visualizer: ResMut<RenetServerVisualizer<200>>, server: Res<RenetServer>) {
    visualizer.update(&server);
    visualizer.show_window(egui_contexts.ctx_mut());
}