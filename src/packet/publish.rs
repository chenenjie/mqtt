use super::FixHeader;
use super::TopicName;
use super::PacketIdentifier;



struct PublishPacket {
    fix_header: FixHeader,
    topic_name: TopicName, 
    packet_identifier: PacketIdentifier, 
    payload: Vec<u8>,
}

