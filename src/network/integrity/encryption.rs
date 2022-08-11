// Relative Modules
pub(crate) mod ipe_dhe;
pub(crate) mod ipe_xtea;

// Standard Uses

// Crate Uses

// External Uses


pub trait EncryptionMethod {
    fn active(&self) -> bool;
    fn prepare(&mut self) -> bool;
    fn activate(&mut self) -> bool;

    fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, ()>;
}

pub trait EncryptionMethodBuilder: EncryptionMethod {
    fn new_boxed() -> Box<dyn EncryptionMethod>;
}
