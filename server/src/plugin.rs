use crate::network::system::{handle_players_input, server_event, server_network_sync};
use crate::system::decor_system::setup_camera;
use crate::system::player_system::{player_jump_control, player_move, player_shoot};
use bevy::app::{App, FixedUpdate, Plugin, Startup, Update};
use bevy::log::error;
use bevy::prelude::{Entity, Events, Resource};
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_renet2::netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerSetupConfig};
use bevy_renet2::prelude::{ClientId, RenetServer};
use game_core::decor::system::setup_ground;
use game_core::network::network::{connection_config, PROTOCOL_ID};
use game_core::network::utils::{get_current_time, get_native_socket, get_socket};
use game_core::player::animation::animate_players;
use game_core::texture::system::{load_player_textures, load_weapon_fx_textures, load_weapon_textures};
use game_core::weapon::animation::animate_weapons;
use game_core::weapon::component::despawn_weapon_fx_out_of_screen_system;
use renet2_visualizer::RenetServerVisualizer;
use std::collections::HashMap;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerLobby::default());
        app.insert_resource(RenetServerVisualizer::<200>::default());
        app.init_resource::<Events<CollisionEvent>>();

        add_netcode_network(app);

        app.add_systems(Update, (
            server_event,
            animate_players,
            animate_weapons,
            player_jump_control,
            player_move,
            player_shoot,
            despawn_weapon_fx_out_of_screen_system
        ));

        app.add_systems(FixedUpdate, (
            server_network_sync,
            handle_players_input,
        ));


        app.add_systems(Startup, (
            load_player_textures,
            load_weapon_textures,
            load_weapon_fx_textures,
            setup_camera,
            setup_ground,
        ));
    }
}

fn add_netcode_network(app: &mut App) {
    app.add_plugins(NetcodeServerPlugin);

    let server = RenetServer::new(connection_config());

    let public_addr = "127.0.0.1:5000".parse().expect("Échec du parsing de l'adresse publique");

    let socket = get_socket(public_addr);

    let current_time: std::time::Duration = get_current_time();

    let server_config = ServerSetupConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        socket_addresses: vec![vec![public_addr]],
        authentication: ServerAuthentication::Unsecure,
    };

    let native_socket = get_native_socket(socket);

    let transport = NetcodeServerTransport::new(server_config, native_socket).unwrap_or_else(|e| {
        error!("Erreur lors de la création du transport serveur");
        panic!("reason : {e}");
    });

    app.insert_resource(server);
    app.insert_resource(transport);
}