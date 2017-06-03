use super::FixHeader;
use super::PacketIdentifier;

struct PubrelPacket{
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}