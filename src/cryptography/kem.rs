// Relative Modules
pub(crate) mod dh;

// Standard Uses

// Crate Uses

// External Uses


pub trait KeyAgreement<T> {
    fn prepare(&mut self, buffer: fn(), length: usize) -> usize;
    fn agree(&mut self, agreed_length: usize, buffer: fn(), length: usize) -> bool;
    // fn shared(&self) -> Box<dyn BlockCipher<BlockSize = usize>>;
}

pub trait KeyAgreementBuilder<T>: KeyAgreement<T> {
    fn new_boxed() -> Box<dyn KeyAgreement<T>>;
}
