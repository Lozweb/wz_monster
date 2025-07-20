use bevy::prelude::{App, Component, Entity, EventReader, IntoScheduleConfigs, Resource, SystemSet, Update};
use bevy_renet2::netcode::{ClientAuthentication, NativeSocket, NetcodeClientPlugin, NetcodeClientTransport, NetcodeTransportError};
use bevy_renet2::prelude::{client_connected, ClientId, RenetClient};
use game_core::network::network_entities::{connection_config, PROTOCOL_ID};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

pub mod component;
pub mod system;

#[derive(Component)]
struct ControlledPlayer;

#[derive(Default, Resource)]
pub struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>,
}

#[derive(Debug, Resource)]
pub struct CurrentClientId(u64);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

pub fn add_netcode_network(app: &mut App) {
    app.add_plugins(NetcodeClientPlugin);
    app.configure_sets(Update, Connected.run_if(client_connected));

    let client = RenetClient::new(connection_config(), false);

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        socket_id: 0,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, NativeSocket::new(socket).unwrap()).unwrap();

    app.insert_resource(client);
    app.insert_resource(transport);
    app.insert_resource(CurrentClientId(client_id));

    #[allow(clippy::never_loop)]
    fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
        for e in renet_error.read() {
            panic!("{}", e);
        }
    }

    app.add_systems(Update, panic_on_error_system);
}