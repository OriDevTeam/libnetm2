// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::algorithms::BlockAlgorithm;

// Library Uses
use cipher::{BlockEncrypt, BlockSizeUser, KeyInit, KeySizeUser};
use blowfish::{BlowfishLE};
use cipher::generic_array::GenericArray;


pub struct BlowfishAlgorithm {
    key: Vec<u8>,
    iv: Vec<u8>
}

impl BlockAlgorithm for BlowfishAlgorithm {
    fn block_size(&self) -> usize {
        BlowfishLE::block_size()
    }

    fn default_key_length(&self) -> usize {
        BlowfishLE::key_size()
    }

    fn iv_length(&self) -> usize {
        // let iv_len = Key::cr
        todo!()
    }

    fn create(self: &mut BlowfishAlgorithm, key: Box<[u8]>, _length: usize, iv: Vec<u8>) {
        self.key = Vec::from(key);
        self.iv = iv;
    }

    fn encrypt(&self, mut data: Box<[u8]>) -> Box<[u8]> {
        let mut plain = GenericArray::from_mut_slice(&mut data);

        let blowfish = BlowfishLE::new_from_slice(&self.key);

        blowfish.unwrap().encrypt_block(&mut plain);
        Box::from(plain.to_vec())
    }

    fn decrypt(&self, mut data: Box<[u8]>) -> Box<[u8]> {
        let mut plain = GenericArray::from_mut_slice(&mut data);

        let twofish = BlowfishLE::new_from_slice(&self.key);

        twofish.unwrap().encrypt_block(&mut plain);
        Box::from(plain.to_vec())
    }
}
