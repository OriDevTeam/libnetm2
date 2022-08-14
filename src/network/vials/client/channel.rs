// Standard Uses
use std::sync::Arc;
use std::mem::size_of;
use std::rc::Rc;

// Crate Uses
use crate::network::connection::settings::ConnectionSettings;
use crate::network::integrity::encryption::{EncryptionMethod};
use crate::network::integrity::synchrony::handshake::{Handshake};
use crate::network::vials::base::net::Net;
use crate::network::integrity::synchrony::handshake::hand::HandBuilder;
use crate::network::integrity::synchrony::handshake::left_hand::{LeftHand};
use crate::network::vials::base::SocketBase;
use crate::network::integrity::synchrony::phase::PhasePacket;
use crate::network::packets::dynamic_manager::DynamicManager;
use crate::network::packets::packet::{Packet, PacketBuilder, PacketHeader, Sender};

// External Uses


pub struct ClientChannel {
    pub settings: ConnectionSettings,
    pub socket: Arc<Net>,
    // pub socket: Net,
    pub manager: Rc<DynamicManager>,

    // synchronization_methods: Vec<Box<dyn SynchronizationMethod>>,
    handshake: Handshake,

    encryption_methods: Vec<Box<dyn EncryptionMethod>>
}

impl ClientChannel {

    pub fn connected(&self) -> bool {
        false
    }

    pub fn connect(settings: ConnectionSettings) -> Self {
        let mut manager = Rc::new(DynamicManager::new());
        let clone = Rc::get_mut(&mut manager).unwrap();

        register_packets(clone);

        let net = Net::connect(&settings, manager.clone()).unwrap();

        let channel = Self {
            settings: settings.clone(),
            socket: Arc::from(net),
            // socket: net,
            manager,

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


pub(crate) fn register_packets(manager: &mut DynamicManager) {
    manager.add_packet(PhasePacket::HEADER, size_of::<PhasePacket>(), PhasePacket::from_bytes_rc);
    manager.add_receiver(PhasePacket::HEADER, receive_phase_packet);
}


// #[receive_packet(MANAGER, PhasePacket)]
fn receive_phase_packet(_packet: Rc<dyn Packet>) {
    let packet = _packet.downcast_ref::<PhasePacket>().unwrap();

    println!("Got Phase packet {:?}", packet);
}


pub enum ClientPacket {
    Phase(PhasePacket)
}

impl Packet for ClientPacket {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
