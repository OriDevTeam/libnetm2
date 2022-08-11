// Relative Modules
pub(crate) mod blowfish;
pub(crate) mod twofish;

// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::algorithms::blowfish::BlowfishAlgorithm;
use crate::cryptography::ciphers::algorithms::twofish::TwofishAlgorithm;

// External Uses


pub trait BlockAlgorithm {
    fn block_size(&self) -> usize;
    fn default_key_length(&self) -> usize;
    fn iv_length(&self) -> usize;

    fn create(&mut self, key: Box<[u8]>, length: usize, iv: Vec<u8>);
    fn encrypt(&self, data: Box<[u8]>) -> Box<[u8]>;
    fn decrypt(&self, data: Box<[u8]>) -> Box<[u8]>;

}

pub trait BlockAlgorithmBuilder: BlockAlgorithm {
    fn new_boxed() -> Box<dyn BlockAlgorithm>;
}

pub trait BlockDetails {}


pub enum Algorithm {
    RC6,
    AES,
    MARS,
    TwoFish(TwofishAlgorithm),
    Serpent,
    CAST256,
    IDEA,
    ThreeDES,
    Camellia,
    SEED,
    RC5,
    Blowfish(BlowfishAlgorithm),
    TEA,
    SKIPJACK,
    SHACAL2,
}

impl Algorithm {
    pub fn pick(hint: i32) -> Algorithm { Self::default() }
}

impl Default for Algorithm {
    fn default() -> Algorithm {
        todo!()

        // TwofishAlgorithm::new()
    }
}

