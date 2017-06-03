use super::FixHeader;
use super::PacketIdentifier;


struct UnsubackPacket{
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}
