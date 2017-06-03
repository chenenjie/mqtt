
use super::FixHeader;
use super::PacketIdentifier;

struct PubrecPacket{
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}