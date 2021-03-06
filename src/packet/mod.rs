use bytes::BytesMut;
use std::string::FromUtf8Error;

trait MqttPacketCodec: Sized {
    type Error;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>;

    fn encode(&self) -> Result<BytesMut, Self::Error>;
}

pub struct FixHeader{
    message_type: u8,
    is_dup: bool,
    qos_level: u8,
    retain: bool,
    remaining_length: u32,
}

impl FixHeader {
    pub fn new(message_type: u8, is_dup: bool, qos_level: u8, retain: bool, remaining_length: u32) -> FixHeader {
        FixHeader{
            message_type: message_type,
            is_dup: is_dup,
            qos_level: qos_level,
            retain: retain,
            remaining_length: remaining_length,
        }
    }
}

enum FixHeaderError{
    RemainingLengthError,
    UnDecodeError(String),
}

impl MqttPacketCodec for FixHeader {
        type Error = FixHeaderError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{
        if bytes.len() < 2 {
            return Err(FixHeaderError::UnDecodeError("fixHeader bytes less than 2 bytes".to_owned()))
        }
        let message_type = (*(bytes.split_to(1)))[0];

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
                if bytes.len() != 0 {
                    return Err(FixHeaderError::UnDecodeError("fix header remain lenght is not enough".to_owned()))  
                }
                let byte = (*(bytes.split_to(1)))[0];
                cur |= ((byte as u32)) & 0x7F << (7 * i);
                if i >= 4 {
                    return Err(FixHeaderError::RemainingLengthError)
                }
                if byte & 0x80 == 0 {
                    break;
                }
            }
            cur
        };

        Ok(FixHeader::new(packet_type, dup_flag, qos_level, retain, remaining_len))
    }

    fn encode(&self) -> Result<BytesMut, Self::Error>{
        Ok(BytesMut::from(&b"hello"[..]))
    }
}

struct TopicName(pub String);

impl MqttPacketCodec for TopicName{
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{
        Ok(TopicName(MqttPacketCodec::decode(bytes)?))
    }

    fn encode(&self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from(&b"hello"[..]))
    }
}

struct PacketIdentifier(pub u16);

pub enum PacketError{
    StringDecodeError,
    NumValidDecodeError,
    StringUTF8ConvertError(FromUtf8Error)
}

impl MqttPacketCodec for String {
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error> {
        if bytes.len() < 2{
            return Err(PacketError::NumValidDecodeError)
        };
        let len = {
            let len_arr = bytes.split_to(2);
            let mut sum = 0u16;
            sum = ((*len_arr)[0] as u16) & 0xFF << 8 | ((*len_arr)[1] as u16);
            sum as usize
        };
        if bytes.len() < len {
            return Err(PacketError::NumValidDecodeError)
        };
        String::from_utf8(bytes.split_to(len).to_vec()).map_err(PacketError::StringUTF8ConvertError)
        
    }

    fn encode(&self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from(&b"enjie"[..]))
    }
}

impl MqttPacketCodec for u8 {
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error> {
        if bytes.len() < 1 {
            return Err(PacketError::NumValidDecodeError)
        }
        Ok((*bytes.split_to(1))[0])
    }

    fn encode(&self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from(&b"enjie"[..]))
    }
}

impl MqttPacketCodec for u16 {
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error> {
        if bytes.len() < 2 {
            return Err(PacketError::NumValidDecodeError)
        }
        let o_byte = bytes.split_to(2);
        let mut result = 0u16;
        result = (((*o_byte)[0] as u16) & 0xFFFF << 8) | (((*o_byte)[1] as u16) & 0xFFFF); 
        Ok(result)
    }

    fn encode(&self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from(&b"enjie"[..]))
    }
}

impl MqttPacketCodec for Vec<u8> {

    type Error = PacketError;

    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error> {

    }

    fn encode(&self) -> Result<BytesMut, Self::Error> {

    }
}