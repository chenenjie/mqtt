use super::FixHeader;
use super::PacketIdentifier;

struct SubackPacket{
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,

}


struct SubackPacketPayload {
    suback_return_code: Vec<SubackReturnCode>,
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SubackReturnCode {
    MaximumQoSLevel0 = 0x00,
    MaximumQoSLevel1 = 0x01,
    MaximumQoSLevel2 = 0x02,
    Failure = 0x80,
}