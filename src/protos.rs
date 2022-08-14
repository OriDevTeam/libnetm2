// Included Modules

pub mod packets {
    include!(concat!(env!("OUT_DIR"), "/network.packets.authentication.rs"));
    include!(concat!(env!("OUT_DIR"), "/network.packets.synchronization.rs"));
}
