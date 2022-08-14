// Relative Modules
pub(crate) mod cipher;
pub(crate) mod packets;


// Standard Uses

// Crate Uses
use crate::network::integrity::encryption::EncryptionMethod;

// External Uses



pub struct DiffieHellmanEphemeral {
    pub ready: bool
}


impl EncryptionMethod for DiffieHellmanEphemeral {
    fn active(&self) -> bool { todo!() }

    fn prepare(&mut self) -> bool { todo!() }

    fn activate(&mut self) -> bool { todo!() }

    fn encrypt(&self, _data: Vec<u8>) -> Result<Vec<u8>, ()> {
        todo!()
    }

    fn decrypt(&self, _data: Vec<u8>) -> Result<Vec<u8>, ()> { todo!() }
}

