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
    fn header() -> u8 { 255 }
}

impl PacketBuilder for Handshake {
    fn from_bytes(_data: Vec<u8>) -> Self {
        todo!()
    }

    fn from_bytes_boxed(_data: Vec<u8>) -> Box<dyn Packet> {
        todo!()
    }
}

