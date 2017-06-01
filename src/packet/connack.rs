use super::FixHeader;

#[derive(Debug)]
struct ConnectAcknowledgeFlag {
    session_present: bool,
}

struct ConnectReturnCode(pub u8);

struct ConnackPacket{
    fix_header: FixHeader,
    clf: ConnectAcknowledgeFlag,
    crc: ConnectReturnCode,    
}