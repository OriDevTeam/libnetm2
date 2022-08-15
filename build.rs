// Standard Uses

// Crate Uses

// External Uses
extern crate prost_build;


fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(&["."]);

    config.type_attribute(
        "network.packets.synchronization.PhasePacket",
        "use netm2_macros::*;\n\
        use crate::network::packets::packet::*;\n\
        #[derive(Packet)]",
    );

    config.compile_protos(&[
        "src/branches/protobufs/network/packets/synchronization/phase.proto",
        "src/branches/protobufs/network/packets/authentication/login.proto"
    ], &["src/"]).unwrap();
}
