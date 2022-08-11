// Standard Uses
use rand::{Rng, thread_rng};

// Crate Uses

// External Uses


pub trait Hand {
    fn shake(&mut self, right_hand: u32) -> bool;

    fn deny(&self);

    fn finalize(&self);

    fn generate_value(&self) -> u64 {
        let mut rng = thread_rng();
        let value = rng.gen_range(0..u64::MAX);

        value
    }
}


pub trait HandBuilder: Hand {
    fn new_boxed() -> Box<dyn Hand>;
}
