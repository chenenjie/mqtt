
trait MqttPacketCodec {
    fn decode();

    fn encode();
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
