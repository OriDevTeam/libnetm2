// Standard Uses
use std::sync::Arc;

// Crate Uses
use crate::network::integrity::encryption::ipe_xtea::IPEXTEA;
use crate::network::packets::packet::{Packet, Receiver, Sender};
use crate::network::vials::base::SocketBase;

// External Uses


impl Receiver for IPEXTEA {
    fn receive_packet(&mut self, packet: Box<dyn Packet>) -> Result<Box<dyn Packet>, ()> {
        todo!()
    }
}

impl Sender for IPEXTEA {
    fn connection(&self) -> Arc<dyn SocketBase> { todo!() }
}

pub struct IPEXTEAKeyAgreement {}

impl Packet for IPEXTEAKeyAgreement {
    fn to_bytes(&self) -> Vec<u8> { todo!() }
}

