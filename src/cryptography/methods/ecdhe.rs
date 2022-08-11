// Relative Modules
pub(crate) mod packets;

// Standard Uses

// Crate Uses
use crate::network::integrity::encryption::{EncryptionMethod, EncryptionMethodBuilder};
use crate::network::packets::packet::{Sender};
use crate::cryptography::methods::ecdhe::packets::ECDHEPublicKeyReceivePacket;

// External Uses
use cipher::rand_core::OsRng;
use p256::{EncodedPoint};
use p256::ecdh::{EphemeralSecret, SharedSecret};


pub struct EllipticCurveDiffieHellman {
    pub active: bool,

    pub private_key: EphemeralSecret,
    pub public_key: EncodedPoint,

    pub other_public_key: Option<EncodedPoint>,

    pub shared: Option<SharedSecret>
}

impl EncryptionMethod for EllipticCurveDiffieHellman {
    fn active(&self) -> bool {
        todo!()
    }

    fn prepare(&mut self) -> bool {
        let packet = ECDHEPublicKeyReceivePacket { public_key: Vec::from(self.public_key.as_bytes()) };
        self.send_packet_boxed(Box::new(packet));

        true
    }

    fn activate(&mut self) -> bool {
        todo!()
    }

    fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, ()> {
        todo!()
    }

    fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, ()> {
        todo!()
    }
}


impl EncryptionMethodBuilder for EllipticCurveDiffieHellman {
    fn new_boxed() -> Box<dyn EncryptionMethod> {
        let private_key = EphemeralSecret::random(&mut OsRng);
        let public_key = EncodedPoint::from(private_key.public_key());

        Box::new(Self {
            active: false,
            private_key,
            public_key,
            other_public_key: None,
            shared: None
        })
    }
}

impl EllipticCurveDiffieHellman {

}