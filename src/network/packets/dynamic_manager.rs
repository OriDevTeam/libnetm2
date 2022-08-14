// Standard Uses
use std::collections::HashMap;
use std::rc::Rc;

// Crate Uses
use crate::network::packets::packet::{Packet};

// External Uses


#[derive(Clone)]
pub struct DynamicManager {
    pub packets: HashMap<u8, (usize, fn(Vec<u8>) -> Rc<dyn Packet>)>,
    pub receivers: HashMap<u8, Vec<fn(Rc<dyn Packet>)>>
}


impl DynamicManager {

    pub fn new() -> Self {
        Self { packets: Default::default(), receivers: Default::default() }
    }

    pub fn add_packet(&mut self, id: u8, _size: usize, _receiver: fn(Vec<u8>) -> Rc<dyn Packet>) {
        self.packets.insert(id, (_size, _receiver));
    }

    pub fn has_packet(&self, id: u8) -> bool {
        self.packets.contains_key(&id)
    }

    pub fn get_packet_size(&self, id: &u8) -> Result<usize, ()>{
        self.packets.get(&id).map(|p| p.0).ok_or(())
    }

    pub fn create_packet(&self, id: u8) -> Rc<dyn Packet> {
        let packet_type = self.packets.get(&id);

        let data: Vec<u8> = Vec::new();

        let packet = packet_type.unwrap().1(data);

        packet
    }

    pub fn add_receiver(&mut self, id: u8, receiver: fn(Rc<dyn Packet>)) {
        self.receivers.entry(id).or_insert_with(Vec::new).push(receiver);
    }

    pub fn construct_packet(&self, header: u8, message: Vec<u8>) -> Result<(), ()> {
        if !self.has_packet(header) {
            println!("No packet type found with the header {}", header);
            return Err(())
        }

        let creator = self.packets.get(&header);
        let packet = (creator.unwrap().1)(message);

        self.dispatch_packet(header, packet);

        Ok(())
    }

    fn dispatch_packet(&self, header: u8, packet: Rc<dyn Packet>) {
        if !self.receivers.contains_key(&header) {
            return
        }

        for receiver in &self.receivers[&header] {
            receiver(packet.clone())
        }
    }
}

