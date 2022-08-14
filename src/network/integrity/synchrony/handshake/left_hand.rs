// Standard Uses
use std::sync::Arc;

// Crate Uses
use crate::network::integrity::synchrony::handshake::hand::{Hand, HandBuilder};
use crate::network::packets::packet::{Packet, PacketHeader, Sender};
use crate::network::vials::base::SocketBase;

// External Uses


pub struct LeftHand {
    pub hand: u32,
    pub time: u64,
}

impl Hand for LeftHand {

    fn shake(&mut self, hand: u32) -> bool {
        self.hand = hand;

        let packet = Box::new(HandshakePacket {
            hand_value: self.hand,
            time: 0,
            delta: 0
        });

        self.send_packet_boxed(packet);

        true
    }

    fn deny(&self) {
        todo!()
    }

    fn finalize(&self) {
        todo!()
    }
}

impl Sender for LeftHand {
    fn connection(&self) -> Arc<dyn SocketBase> {
        todo!()
    }

    fn send_packet_boxed(&mut self, packet: Box<dyn Packet>) {
        let mut packet_bytes = vec!();

        let bytes = Vec::from(unsafe {
            std::mem::transmute::<u32, [u8; 4]>(self.hand)
        });

        packet_bytes.push(bytes);

        todo!()
        // self.base().send_bytes(packet_bytes);
    }
}

impl HandBuilder for LeftHand {
    fn new_boxed() -> Box<dyn Hand> {
        Box::new(LeftHand { hand: 0, time: 0 })
    }
}


pub(crate) struct HandshakePacket {
    hand_value: u32,
    time: u32,
    delta: u64
}

impl Packet for HandshakePacket {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl PacketHeader for HandshakePacket {
    const HEADER: u8 = 255;
}
