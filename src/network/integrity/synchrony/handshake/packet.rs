// Standard Uses

// Crate Uses
use crate::network::integrity::synchrony::handshake::Handshake;
use crate::network::packets::packet::{Packet, PacketBuilder, PacketHeader};

// External Uses


impl Packet for Handshake {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl PacketHeader for Handshake {
    const HEADER: u8 = 255;
}


