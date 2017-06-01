use super::FixHeader;


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

struct TopicName(pub String);

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




