// Standard Uses

// Crate Uses
use crate::network::packets::packet::Packet;

// External Uses


pub struct Manager {
    pub packets: Vec<Box<dyn Packet>>
}

impl Manager {
    pub(crate) fn new() -> Manager {
        Manager { packets: vec![] }
    }

}

impl Manager {

    pub fn register<T>(&mut self, packet: Box<dyn Packet>) {
        self.packets.push(packet);
    }

    pub fn get_type(&self, _header: u8) -> Result<Box<dyn Packet>, ()> {
        todo!()
    }
}


pub trait Receiver {
    // fn receive_packet(&self, packet: &dyn Self);
}

