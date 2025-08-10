use crate::animation::player_animation;
use crate::network::system::client_event;
use crate::network::{ClientLobby, Connected, PlayerMapping, ProjectileMapping};
use crate::player_input::{send_input, update_mouse_coords, MainCamera};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::log::error;
use bevy::prelude::{Camera2d, Commands, EventReader, IntoScheduleConfigs};
use bevy_renet2::netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeTransportError};
use bevy_renet2::prelude::{client_connected, RenetClient};
use game_core::decor::system::setup_ground;
use game_core::network::network::{connection_config, PROTOCOL_ID};
use game_core::network::utils::{get_current_time, get_native_socket, get_socket};
use game_core::player::component::{AimDirection, CurrentClientId, MouseWorldCoords, PlayerInput};
use game_core::texture::system::{load_player_textures, load_weapon_fx_textures, load_weapon_textures};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientLobby::default());
        app.insert_resource(PlayerMapping::default());
        app.insert_resource(ProjectileMapping::default());
        app.insert_resource(PlayerInput::default());
        app.insert_resource(MouseWorldCoords::default());
        app.insert_resource(AimDirection::default());

        add_netcode_network(app);

        app.add_systems(Update, (
            client_event,
            send_input,
            update_mouse_coords,
            player_animation,
        ).in_set(Connected));

        app.add_systems(Startup, (
            setup_camera,
            setup_ground,
            load_player_textures,
            load_weapon_textures,
            load_weapon_fx_textures,
        ));
    }
}
fn add_netcode_network(app: &mut App) {
    app.add_plugins(NetcodeClientPlugin);
    app.configure_sets(Update, Connected.run_if(client_connected));

    let client = RenetClient::new(connection_config(), false);

    let server_addr = "127.0.0.1:5000".parse().expect("Échec du parsing de l'adresse publique");
    let socket_addr = "127.0.0.1:0".parse().expect("Échec du parsing de l'adresse du socket");
    let socket = get_socket(socket_addr);

    let current_time: std::time::Duration = get_current_time();

    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        socket_id: 0,
        server_addr,
        user_data: None,
    };

    let native_socket = get_native_socket(socket);

    let transport = NetcodeClientTransport::new(current_time, authentication, native_socket).unwrap_or_else(|e| {
        error!("Échec de la création du transport client");
        panic!("reason : {e}");
    });

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

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}