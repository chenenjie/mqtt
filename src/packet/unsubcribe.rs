use super::FixHeader;
use super::PacketIdentifier;
use super::TopicName;


struct UnsubscribePacket{
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,
    payload: UnsubscribePacketPayload,
}


struct UnsubscribePacketPayload{
    unsubcribes: Vec<TopicName>,
}