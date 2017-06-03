use super::FixHeader;
use super::PacketIdentifier;
use super::TopicName;


struct SubscribePacket{
    fix_header: FixHeader,
    packet_identifier: PacketIdentifier,
    payload: SubscribePacketPayload,
}

struct RequestedQoS(pub u8)

struct SubscribePacketPayload{
    subscribes: Vec<(TopicName, RequestedQoS)>;
}
