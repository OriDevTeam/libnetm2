// Relative Modules

// Standard Uses

// Crate Uses
use crate::network::integrity::encryption::{EncryptionMethod, EncryptionMethodBuilder};
use crate::cryptography::methods::dhe::cipher::ImprovedPacketEncryption;

// External Uses


pub struct IPEDiffieHellman {
    activated: bool
}

impl EncryptionMethod for ImprovedPacketEncryption<IPEDiffieHellman> {
    fn active(&self) -> bool { self.activated }

    fn prepare(&mut self) -> bool {
        todo!()
    }

    fn activate(&mut self) -> bool { todo!() }

    fn encrypt(&self, _data: Vec<u8>) -> Result<Vec<u8>, ()> {
        if !self.active() {
            panic!("Cannot encrypt data if IPE is not ready");
        }

        todo!()
    }

    fn decrypt(&self, _data: Vec<u8>) -> Result<Vec<u8>, ()> {
        if !self.active() {
            panic!("Cannot decrypted data if IPE is not ready");
        }

        todo!()
    }
}


impl EncryptionMethodBuilder for ImprovedPacketEncryption<IPEDiffieHellman> {
    fn new_boxed() -> Box<dyn EncryptionMethod> {
        todo!()
    }
}
