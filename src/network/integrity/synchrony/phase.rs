// Standard Uses
use std::rc::Rc;

// Crate Uses
use crate::network::packets::packet::{Packet, PacketBuilder, PacketHeader};

// External Uses


#[derive(Debug)]
pub enum Phase {
    Close,
    Handshake,
    Login,
    Auth,
    Select,
    Loading,
    Game,
    Dead
}


#[derive(Debug)]
pub struct PhasePacket {
    pub phase: Phase
}

impl Packet for PhasePacket {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl PacketHeader for PhasePacket {
    const HEADER: u8 = 253;
}


// #[receive_packet(MANAGER, PhasePacket)]
pub(crate) fn receive_phase_packet(packet: Rc<dyn Packet>) {
    let packet = packet.downcast_ref::<PhasePacket>().unwrap();

    println!("Got Phase packet {:?}", packet);
}

