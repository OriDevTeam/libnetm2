// Standard Uses
use std::sync::Arc;
use core::option::Option;

// Crate Uses
use crate::network::integrity::encryption::EncryptionMethod;
use crate::network::packets::packet::{Packet, PacketBuilder, Receiver, Sender};
use crate::cryptography::methods::ecdhe::EllipticCurveDiffieHellman;
use crate::network::vials::base::SocketBase;

// External Uses
use p256::PublicKey;


impl Sender for EllipticCurveDiffieHellman {
    fn connection(&self) -> Arc<dyn SocketBase> { todo!() }
}

impl Receiver for EllipticCurveDiffieHellman {
    fn receive_packet(&mut self, packet: Box<dyn Packet>) -> Result<Box<dyn Packet>, ()> {
        let concrete_packet = packet.downcast_ref::<ECDHEPublicKeyReceivePacket>().unwrap();

        let other_public_key = PublicKey::from_sec1_bytes(&concrete_packet.public_key);

        if other_public_key.is_err() {
            println!("Received ECDHE Public Key is not valid")
        }

        self.shared = Option::from(self.private_key.diffie_hellman(&other_public_key.unwrap()));

        self.activate();

        Ok(packet)
    }
}

#[derive(Debug)]
pub struct ECDHEPublicKeyReceivePacket {
    pub public_key: Vec<u8>
}

impl Packet for ECDHEPublicKeyReceivePacket {
    fn to_bytes(&self) -> Vec<u8> { todo!() }
}

