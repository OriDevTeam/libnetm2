// Relative Modules
pub(crate) mod group;

// Standard Uses

// Crate Uses
use crate::cryptography::kem::{KeyAgreement, KeyAgreementBuilder};

// External Uses



pub struct DiffieHellman {
    // right_private_key: i32,
    // left_private_key: i32,

    pub diffie: i32,
    agreed_value_length: usize
}

impl<T> KeyAgreement<T> for DiffieHellman {
    fn prepare(&mut self, _buffer: fn(), _length: usize) -> usize {
        // let (p, g, q) = make_modp_group_1024bit_with_160bit_primer_order_subgroup();

        todo!()
    }

    fn agree(&mut self, _agreed_length: usize, _buffer: fn(), _length: usize) -> bool {

        if _agreed_length != self.agreed_value_length {
            return false
        }

        let left_key_length: usize = 0; // DH2.StaticPublicKeyLength();
        let right_key_length: usize = 0; // DH2.EphemeralPublicKeyLength();

        if _length != left_key_length + right_key_length {
            return false;
        }

        // self.shared().new();

        true
    }

    // fn shared(&self) { todo!() }
}

impl<T> KeyAgreementBuilder<T> for DiffieHellman {
    fn new_boxed() -> Box<dyn KeyAgreement<T>> {
        Box::new(Self {
            diffie: 0,
            // right_private_key: 0,
            // left_private_key: 0,
            agreed_value_length: 0
        })
    }
}
