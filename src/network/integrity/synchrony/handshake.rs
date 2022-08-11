// Relative Modules
pub(crate) mod hand;
pub(crate) mod packet;
pub(crate) mod right_hand;
pub(crate) mod left_hand;


// Standard Uses

// Crate Uses
use crate::network::integrity::synchrony::handshake::hand::{Hand};
use crate::network::integrity::synchrony::handshake::left_hand::LeftHand;
use crate::network::integrity::synchrony::handshake::right_hand::RightHand;
use crate::network::integrity::synchrony::SynchronizationMethod;

// External Uses


pub struct Handshake {
    // pub side: Side,
    pub hand: Box<dyn Hand>,
    shook: bool
}

impl Handshake {

    pub fn shook(&self) -> bool {
        self.shook
    }

    /*
    pub fn new(side: Side) -> Handshake {
        Handshake {
            side,
            hand: RightHand::new_boxed(),
            shook: false
        }
    }
    */

    pub fn new2(hand: Box<dyn Hand>) -> Handshake {
        Handshake {
            hand,
            shook: false
        }
    }

}

impl Handshake {
    pub(crate) fn new_boxed(_side: Side) -> Box<dyn SynchronizationMethod> {
        todo!()
    }
}

pub enum Side {
    Left(LeftHand),
    Right(RightHand)
}


impl SynchronizationMethod for Handshake {

}
