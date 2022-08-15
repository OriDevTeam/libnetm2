// Standard Uses
use std::mem::size_of;

// Crate Uses
use crate::network::integrity::synchrony::phase::{PhasePacket, receive_phase_packet};
use crate::network::packets::dynamic_manager::DynamicManager;
use crate::network::packets::packet::{PacketBuilder, PacketHeader};

// External Uses

pub(crate) fn register_packets(manager: &mut DynamicManager) {
    manager.add_packet(PhasePacket::HEADER, size_of::<PhasePacket>(), PhasePacket::from_bytes_rc);
    manager.add_receiver(PhasePacket::HEADER, receive_phase_packet);
}
