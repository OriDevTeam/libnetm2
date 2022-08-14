// Standard Uses
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

// Crate Uses
use crate::network::vials::base::SocketBase;
use crate::protos::packets;

// External Uses
use downcast_rs::{Downcast, impl_downcast};
use prost::{Message};
use prost::alloc::{vec, vec::Vec};


pub trait Packet: Downcast {
    fn to_bytes(&self) -> Vec<u8>;
}
impl_downcast!(Packet);

pub trait PacketHeader: Packet {
    const HEADER: u8;
}

pub trait PacketBuilder: Packet {
    fn from_bytes(data: Vec<u8>) -> Self;
    fn from_bytes_boxed(data: Vec<u8>) -> Box<dyn Packet> { todo!() }
    fn from_bytes_rc(data: Vec<u8>) -> Rc<dyn Packet> {
        todo!()
    }
}

impl Display for Box<dyn Packet> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", "Header")
    }
}


pub trait Receiver {
    fn receive_packet(&mut self, packet: Box<dyn Packet>) -> Result<Box<dyn Packet>, ()>;
}

pub trait Sender {
    fn connection(&self) -> Arc<dyn SocketBase>;

    fn send_packet_boxed(&mut self, _packet: Box<dyn Packet>) {
        /*
        if !self.manager.has_packet(packet.borrow()) {
            println!("Packet is not registered");
        }
        */

        // let bytes = vec!();

        // self.connection().send_bytes(bytes);
    }
}


pub fn create() {
    let phase = packets::PhasePacket::default();

    let mut encoded = phase.encode_to_vec();
    let decoded = deserialize(&encoded);

}

pub fn deserialize(buf: &[u8]) -> Result<packets::PhasePacket, prost::DecodeError> {
    packets::PhasePacket::decode(buf)
}

impl<T: prost::Message + 'static> Packet for T {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
