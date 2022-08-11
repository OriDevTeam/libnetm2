// Standard Uses
use std::sync::Arc;

// Crate Uses
use crate::network::connection::settings::ConnectionSettings;
use crate::network::integrity::encryption::{EncryptionMethod};
use crate::network::integrity::synchrony::handshake::{Handshake};
use crate::network::vials::base::net::Net;
use crate::network::integrity::synchrony::handshake::hand::HandBuilder;
use crate::network::integrity::synchrony::handshake::left_hand::{HandshakePacket, LeftHand};
use crate::network::vials::base::SocketBase;
use crate::network::integrity::synchrony::phase::PhasePacket;
use crate::network::packets::dynamic_manager::DynamicManager;
use crate::network::packets::packet::{Packet, PacketBuilder, PacketHeader, Sender};

// External Uses


pub struct ClientChannel {
    pub settings: ConnectionSettings,
    pub socket: Arc<Net>,
    pub manager: DynamicManager,

    // synchronization_methods: Vec<Box<dyn SynchronizationMethod>>,
    handshake: Handshake,

    encryption_methods: Vec<Box<dyn EncryptionMethod>>
}

impl ClientChannel {

    pub fn connected(&self) -> bool {
        false
    }

    pub fn connect(settings: ConnectionSettings) -> Self {
        let net = Net::connect(&settings).unwrap();

        let channel = Self {
            settings: settings.clone(),
            socket: Arc::from(net),
            manager: DynamicManager::new(),

            // handshake: Handshake::new(Side::Left),
            handshake: Handshake::new2(LeftHand::new_boxed()),

            encryption_methods: vec![
            ]
        };

        channel
    }

    pub fn prepare(&mut self) {
    }
}

impl Sender for ClientChannel {
    fn connection(&self) -> Arc<dyn SocketBase> {
        self.socket.clone()
    }
}


pub(crate) fn register_packets(mut client: ClientChannel) {
    client.manager.add_packet(PhasePacket::header(), PhasePacket::from_bytes_boxed);
    client.manager.add_receiver(HandshakePacket::header(), receive_phase_packet);

    // client.manager.add_packet
}


// #[receive_packet(MANAGER, TestPacket)]
fn receive_phase_packet(_packet: Box<dyn Packet>) {

}


