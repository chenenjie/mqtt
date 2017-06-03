use super::FixHeader;
use super::PacketIdentifier;


struct PubcompPacket{
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}

 