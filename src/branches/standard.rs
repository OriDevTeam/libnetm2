// Included Modules
pub mod standard_packets {
    include!(concat!(env!("OUT_DIR"), "/network.packets.authentication.rs"));
    include!(concat!(env!("OUT_DIR"), "/network.packets.synchronization.rs"));
}

// Standard Uses

// Crate Uses
use crate::network::packets::packet::Packet;

// External Uses
use enum_dispatch::enum_dispatch;
use strum::EnumIter;
use strum::IntoEnumIterator;


#[enum_dispatch(Packet)]
#[derive(EnumIter)]
enum StandardPacket {
    Phase(standard_packets::PhasePacket)
}

impl StandardPacket {
    fn find(header: u8) {
        for p in StandardPacket::iter() {
        }
    }
}
