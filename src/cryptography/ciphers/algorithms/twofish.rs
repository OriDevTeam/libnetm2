// Standard Uses

// Crate Uses
use crate::cryptography::ciphers::algorithms::{BlockAlgorithm, BlockAlgorithmBuilder};


// Library Uses
use cipher::{BlockEncrypt, BlockSizeUser, KeyInit, KeySizeUser};
use cipher::generic_array::GenericArray;
use twofish::Twofish;


pub struct TwofishAlgorithm {
    key: Vec<u8>,
    iv: Vec<u8>
}

impl BlockAlgorithm for TwofishAlgorithm {
    fn block_size(&self) -> usize {
        Twofish::block_size()
    }

    fn default_key_length(&self) -> usize {
        Twofish::key_size()
    }

    fn iv_length(&self) -> usize {
        todo!()
    }

    fn create(&mut self, key: Box<[u8]>, _length: usize, iv: Vec<u8>) {
        self.key = Vec::from(key);
        self.iv = iv;
    }

    fn encrypt(&self, mut data: Box<[u8]>) -> Box<[u8]> {
        let mut plain = GenericArray::from_mut_slice(&mut data);

        let twofish = Twofish::new_from_slice(&self.key);

        twofish.unwrap().encrypt_block(&mut plain);
        Box::from(plain.to_vec())
    }

    fn decrypt(&self, mut data: Box<[u8]>) -> Box<[u8]> {
        let mut plain = GenericArray::from_mut_slice(&mut data);

        let twofish = Twofish::new_from_slice(&self.key);

        twofish.unwrap().encrypt_block(&mut plain);
        Box::from(plain.to_vec())
    }
}


impl BlockAlgorithmBuilder for TwofishAlgorithm {
    fn new_boxed() -> Box<dyn BlockAlgorithm> {

        todo!()

        /*
        Box::new(TwofishAlgorithm {
            blowfish: (),
            stream: Box::new(())
        })
         */
    }
}
