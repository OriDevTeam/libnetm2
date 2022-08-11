// Standard Uses
use std::collections::HashMap;
use std::ops::Index;

// Crate Uses
use crate::network::packets::packet::{Packet};

// External Uses


pub struct DynamicManager {
    pub packets: HashMap<u8, fn(Vec<u8>) -> Box<dyn Packet>>,
    pub receivers: HashMap<u8, Vec<fn(Box<dyn Packet>)>>
}


impl DynamicManager {

    pub fn new() -> Self {
        Self { packets: Default::default(), receivers: Default::default() }
    }

    pub fn add_packet(&mut self, id: u8, receiver: fn(Vec<u8>) -> Box<dyn Packet>) {
        self.packets.insert(id, receiver);
    }

    pub fn has_packet(&self, type_: u8) -> bool {
        self.packets.contains_key(&type_)
    }

    pub fn create_packet(&self, id: u8) -> Box<dyn Packet> {
        let packet_type = self.packets.get(&id);

        let data: Vec<u8> = Vec::new();

        let packet = packet_type.unwrap()(data);

        packet
    }

    pub fn add_receiver(&mut self, id: u8, _receiver: fn(Box<dyn Packet>)) {
        if self.receivers.index(&id).is_empty() {  // TODO: Check if vector exists create if not

        }

        // self.receivers.index(&id).push(receiver);
    }
}
