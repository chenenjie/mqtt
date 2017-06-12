use super::FixHeader;
use super::MqttPacketCodec;
use super::PacketError;

#[derive(Debug)]
struct ConnectAcknowledgeFlag {
    session_present: bool,
}

impl MqttPacketCodec for ConnectAcknowledgeFlag {
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{
        if bytes.len() < 1 {
            return PacketError::NumValidDecodeError;
        }
        let byte = (*(bytes.split_to(1)[0]);
        let session_present = (byte & 0x01 == 0x01);

        let flag = ConnectAcknowledgeFlag{
            session_present: session_present,
        };
        Ok(flag)
    } 

    fn encode(&self) -> Result<BytesMut, Self::Error>{

        Ok(BytesMut::from(&b"enjie"[..]))
    }
}

struct ConnectReturnCode(pub u8);

impl MqttPacketCodec for ConnectReturnCode {
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{
        Ok(ConnectReturnCode(MqttPacketCodec::decode(bytes)?))
    }

    fn encode(&self) -> Result<BytesMut, Self::Error>{
        Ok(BytesMut::from(&b"enjie"[..]))
    }
}

struct ConnackPacket{
    fix_header: FixHeader,
    clf: ConnectAcknowledgeFlag,
    crc: ConnectReturnCode,    
}

impl MqttPacketCodec for ConnackPacket {
    
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{
        let fix_header = MqttPacketCodec::decode(bytes)?;
        let clf = MqttPacketCodec::decode(bytes)?;
        let crc = MqttPacketCodec::decode(bytes)?;

        ConnackPacket{
            fix_header: fix_header,
            clf: clf,
            crc: crc,
        }
    }

    fn encode(&self) -> Result<BytesMut, Self::Error>{
        Ok(BytesMut::from(&b"enjie"[..]))
    }
}