// Standard Uses

// Crate Uses
use crate::cryptography::kem::{KeyAgreement, KeyAgreementBuilder};
use crate::cryptography::kem::dh::DiffieHellman;
use crate::cryptography::ciphers::algorithms::Algorithm;

// External Uses


pub struct ImprovedPacketEncryption<T> {
    pub(crate) activated: bool,
    key_agreement: Box<dyn KeyAgreement<T>>,
}

impl ImprovedPacketEncryption<DiffieHellman> {
    pub fn new() -> Self {
        Self { activated: false, key_agreement: DiffieHellman::new_boxed() }
    }

    pub fn prepare(mut self, buffer: fn(), length: usize) -> usize {
        let agreed_length = self.key_agreement.prepare(buffer, length);

        if agreed_length == 0 {
            drop(self.key_agreement)
        }

        agreed_length
    }

    pub fn activate(mut self, polarity: bool, agreed_length: usize, buffer: fn(), length: usize) -> bool {
        if self.activated {
            return false
        }

        if self.key_agreement.agree(agreed_length, buffer, length) {
            self.activated = self.setup(polarity);
        }

        drop(self.key_agreement);

        return self.activated
    }

    fn setup(&self, _polarity: bool) -> bool {
        /*
        let shared = self.key_agreement.shared();

        if shared.size() < 2 {
            return false
        }

        let left_block = self.make_block().unwrap();
        let right_block = self.make_block().unwrap();
        */

        /*
        let offset = min(left_block.key_length, shared.size() - right_block.key_length);
        right_block.key.assign(shared.ptr() as usize + offset, right_block.key_length);

        let mut offset = shared.size() - left_block.iv_length;
        left_block.iv.assign(shared.ptr() as usize + offset);

        if offset < right_block.iv_length { offset = 0; }
        else { offset = offset - right_block.iv_length; }

        right_block.iv.assign(shared.ptr() as usize + offset, right_block.iv_length);

        if polarity {
            // self.key_agreement.set_encoder(algo);
            // self.key_agreement.set_decode(algo);
        } else {
            // self.key_agreement.set_encoder(algo);
            // self.key_agreement.set_decode(algo);
        }
        */

        true
    }

    fn make_block<T>(&self) -> Result<ProcessData, &str> {
        /*
        let shared = self.key_agreement.shared();

        let hint = (shared.ptr() as usize % shared.size()) as i32;
        let algorithm = BlockAlgorithmPicker::pick(hint).borrow();

        let key_length = algorithm.default_key_length();
        let iv_length = algorithm.block_size();

        if shared.size() < key_length {
            return Err("Invalid key length")
        }

        if shared.size() < iv_length {
            return Err("Invalid IV length")
        }

        Ok(ProcessData {
            algorithm,
            key_length,
            iv_length,
            key: TempKey {},
            iv: TempKey {}
        })
        */

        todo!()
    }

    pub fn encrypt() {
        todo!()
    }

    pub fn decrypt() {
        todo!()
    }

    pub fn is_key_prepared() -> bool {
        todo!()
    }

}


pub struct ProcessData {
    algorithm: Algorithm,
    key_length: usize,
    iv_length: usize,
}


