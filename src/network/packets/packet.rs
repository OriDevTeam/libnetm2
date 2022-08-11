// Standard Uses
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use downcast_rs::{Downcast, impl_downcast};

// Crate Uses
use crate::network::vials::base::SocketBase;

// External Uses


/*
pub struct Field {

}

pub struct Value {

}


pub struct Packet {
    pub header: u8,
    pub fields: Vec<Field>,
    pub values: Vec<Value>
}

impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(message: Vec<u8>) -> Packet {

        Packet {
            header: 0,
            fields: vec![],
            values: vec![]
        }
    }
}
*/

pub trait Packet: Downcast {
    fn to_bytes(&self) -> Vec<u8>;
}
impl_downcast!(Packet);

pub trait PacketHeader: Packet {
    fn header() -> u8;
}

pub trait PacketBuilder: Packet {
    fn from_bytes(data: Vec<u8>) -> Self;
    fn from_bytes_boxed(data: Vec<u8>) -> Box<dyn Packet> { todo!() }
}

impl Display for Box<dyn Packet> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", "Header")
    }
}


pub trait Receiver {
    fn receive_bytes(&self, data: Vec<u8>) { todo!() }
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
