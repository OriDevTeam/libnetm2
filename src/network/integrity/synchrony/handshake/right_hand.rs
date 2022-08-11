// Standard Uses

// Crate Uses
use crate::network::integrity::synchrony::handshake::hand::{Hand, HandBuilder};

// External Uses


pub struct RightHand {
    pub hand: u64,
    pub proposal_time: u64,
}


impl Hand for RightHand {
    fn shake(&mut self, hand: u32) -> bool {
        todo!()
    }

    fn deny(&self) {
        todo!()
    }

    fn finalize(&self) {
        todo!()
    }
}



impl HandBuilder for RightHand {
    fn new_boxed() -> Box<dyn Hand> {
        Box::new(Self { hand: 0, proposal_time: 0 })
    }
}
