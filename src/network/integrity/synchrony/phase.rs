// Standard Uses

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

impl PacketBuilder for PhasePacket {
    fn from_bytes(data: Vec<u8>) -> Self {
        todo!()
    }

    fn from_bytes_boxed(data: Vec<u8>) -> Box<dyn Packet> {
        todo!()
    }
}
