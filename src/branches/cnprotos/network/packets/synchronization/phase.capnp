interface Packet {
    const HEADER :UInt8;
}

struct PhasePacket: Packet {
    const HEADER :UInt8 = 0;

    phase @0 :UInt8;
}
# Phase Synchronization Packet
