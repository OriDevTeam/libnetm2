// Standard Uses

// Crate Uses
use crate::network::packets::packet::{Packet, PacketBuilder, PacketHeader};

// External Uses


pub struct IPEKeyAgreementPacket {
    // agreed_length: u16,
    // data_length: i16,
    // data: [u8; IPEKeyAgreementPacket::MAXIMUM_DATA_LENGTH]
}

impl IPEKeyAgreementPacket {
    // const MAXIMUM_DATA_LENGTH: usize = u8::MAX as usize;
}

impl Packet for IPEKeyAgreementPacket {
    fn to_bytes(&self) -> Vec<u8> { todo!() }
}

impl PacketHeader for IPEKeyAgreementPacket {
    const HEADER: u8 = 250;
}

impl PacketBuilder for IPEKeyAgreementPacket {
    fn from_bytes(_data: Vec<u8>) -> Self { todo!() }
    fn from_bytes_boxed(_data: Vec<u8>) -> Box<dyn Packet> { todo!() }
}

////////////////////////////////////

pub struct IPEKeyAgreedPacket {}
impl IPEKeyAgreedPacket {}

impl Packet for IPEKeyAgreedPacket {
    fn to_bytes(&self) -> Vec<u8> { todo!() }
}

impl PacketHeader for IPEKeyAgreedPacket {
    const HEADER: u8 = 251;
}

impl PacketBuilder for IPEKeyAgreedPacket {
    fn from_bytes(_data: Vec<u8>) -> Self { todo!() }
    fn from_bytes_boxed(_data: Vec<u8>) -> Box<dyn Packet> { todo!() }
}
