use bevy::log::error;
use bevy_renet2::netcode::NativeSocket;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

pub fn get_current_time() -> std::time::Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_else(|e| {
        error!("Erreur lors de la récupération du temps système: {e}");
        std::time::Duration::from_secs(0)
    })
}

pub fn get_socket(socket_address: SocketAddr) -> UdpSocket {
    match UdpSocket::bind(socket_address) {
        Ok(s) => s,
        Err(e) => panic!("Erreur lors de la création du socket UDP: {e}"),
    }
}

pub fn get_native_socket(udp_socket: UdpSocket) -> NativeSocket {
    match NativeSocket::new(udp_socket) {
        Ok(s) => s,
        Err(e) => panic!("Erreur lors de la création du socket natif: {e}"),
    }
}
