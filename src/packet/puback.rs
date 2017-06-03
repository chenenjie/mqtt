
use super::FixHeader;
use super::PacketIdentifier;

struct PubackPacket {
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}