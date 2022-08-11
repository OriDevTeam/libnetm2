// Standard Uses

// Crate Uses

// External Uses


// const ENCRYPTION_KEY: u32 = "1234abcd5678efgh".parse().unwrap();
// const ENCRYPTION_KEY: u32 = { || "testtesttesttesttest".parse().unwrap() };
// const DECRYPTION_KEY: u32 = ENCRYPTION_KEY;


use crate::network::integrity::encryption::EncryptionMethod;

pub struct IPEXTEA {}

impl EncryptionMethod for IPEXTEA {
    fn active(&self) -> bool { todo!() }

    fn prepare(&mut self) -> bool { todo!() }

    fn activate(&mut self) -> bool { todo!() }

    fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, ()> { todo!() }

    fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, ()> { todo!() }
}
