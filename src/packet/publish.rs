use super::FixHeader;
use super::TopicName;
use super::PacketIdentifier;
use super::MqttPacketCodec;
use super::PacketError;


struct PublishPacket {
    fix_header: FixHeader,
    topic_name: TopicName, 
    packet_identifier: PacketIdentifier, 
    payload: Vec<u8>,
}


impl MqttPacketCodec for PublishPacket {
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{
        let fix_header = MqttPacketCodec::decode(bytes)?;
        let topic_name = MqttPacketCodec::decode(bytes)?;
        let packet_identifier = MqttPacketCodec::decode(bytes)?;
        let payload = MqttPacketCodec::decode(bytes)?;

        PublishPacket {
            fix_header: fix_header,
            topic_name: topic_name,
            packet_identifier: packet_identifier,
            payload: payload,
        }
    }

    fn encode(&self) -> Result<BytesMut, Self::Error>{
        
    }
}

