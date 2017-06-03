use bytes::BytesMut;

trait MqttPacketCodec {
    type Error;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>;

    fn encode(&self) -> Result<BytesMut, Self::Error>;
}

pub struct FixHeader{
    message_type: u8,
    is_dup: bool,
    qos_level: u8,
    retain: u8,
}

impl FixHeader {
    pub fn new(message_type: u8, is_dup: bool, qos_level: u8, retain: u8, remaining_length: i32) -> FixHeader {
        FixHeader{
            message_type: message_type,
            is_dup: is_dup,
            qos_level: qos_level,
            retain: retain,
            remaining_length: remaining_length,
        }
    }
}

impl MqttPacketCodec for FixHeader {
        type Error;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{

        let message_type = *(src.split_to(1))[0];

        let packet_type = {
            message_type >> 4
        };
        let dup_flag = {
            message_type & 0x08 == 8
        };
        let qos_level = {
            message_type & 0x06 >> 1
        };
        let retain = {
            message_type & 0x01 != 0
        };
        let remaining_len = {
            let mut cur = 0u32;
            for i in 0.. {
                let byte = *(src.split_to(1))[0];
                cur |= ((byte as u32)) & 0x7F << (7 * i);
                if i >= 4 {
                    //TODO            
                }
                if byte & 0x80 == 0 {
                    break;
                }
            }
            cur
        }

        Ok(FixHeader::new(packet_type, dup_flag, qos_level, retain, remaining_len))
    }

    fn encode(&self) -> Result<BytesMut, Self::Error>{

    }
}

struct TopicName(pub String);

struct PacketIdentifier(pub u16);
