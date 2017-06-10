use super::FixHeader;
use super::TopicName;
use super::MqttPacketCodec;
use super::PacketError;

enum ConnectPackectError{
    ProtocolNameUnicodeError,    
}

struct ProtocolName(pub String);

struct ProtocolLevel(pub u8);

struct ConnectFlag{
    user_name: bool,
    password: bool,
    will_retain: bool,
    will_QoS: u8,
    will_flag: bool,
    clean_session: bool,
    reserved: bool,
}

struct KeepAlive(pub u16);


#[derive(Debug)]
struct ConnectPackectPayload {
    client_identifier: String,
    will_topic: Option<TopicName>,
    will_message: Option<String>,
    user_name: Option<String>,
    password: Option<String>,
}

struct ConnectPackect {
    fix_header: FixHeader,
    protocol_name: ProtocolName,
    protocol_level: ProtocolLevel,
    connect_flag: ConnectFlag,
    keep_alive: KeepAlive,
    payload: ConnectPackectPayload,
}

impl MqttPacketCodec for ConnectPackect{
    type Error;
    fn decode(bytes: BytesMut) -> Result<Self, Self::Error> {
        let fix_header = MqttPacketCodec::decode(bytes)?;
        let protocol_name = MqttPacketCodec::decode(bytes)?;
        let protocol_level = MqttPacketCodec::decode(bytes)?;
        let connect_flag = MqttPacketCodec::decode(bytes)?;
        let keep_alive = MqttPacketCodec::decode(bytes)?;
        
        let client_identifier = MqttPacketCodec::decode(bytes)?;
        // let will_topic = 
        



    }

    fn encode(&self) -> Result<BytesMut, Self::Error> {

    }
}

impl MqttPacketCodec for ConnectFlag{
    type Error = PacketError;
    fn decode(bytes: &mut BytesMut) -> Result<Self, Self::Error>{
        if bytes.len() < 1 {
            return Err(PacketError::StringDecodeError)
        }
        let byte = *(bytes.split_to(1))[0]; 
        let user_name = ((byte & 0x80) == 0x80);
        let password = ((byte & 0x40) == 0x40);
        let will_retain = ((byte & 0x20) == 0x20);
        let will_QoS = ((byte & 0x18) >> 3);
        let will_flag = ((byte & 0x04) == 0x04);
        let clean_session = ((byte & 0x02) == 0x02);
        let reserved = ((byte & 0x01)) == 0x01);

        let connect_flag = ConnectFlag{
            user_name: user_name,
            password: password,
            will_retain: will_retain,
            will_flag: will_flag,
            will_QoS: will_QoS,
            clean_session: clean_session,
            reserved: reserved,
        };
        Ok(connect_flag)
    }

    fn encode(&self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from(&b"enjie"[..]))
    }
}
