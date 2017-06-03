extern crate tokio_io;
extern crate tokio_core;
extern crate bytes;
extern crate futures;
extern crate mqtt;

use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use std::sync::mpsc::channel;
use std::io;
use bytes::BytesMut;
use tokio_io::codec::{Framed, Encoder, Decoder};
use mqtt::packet::FixHeader;


struct MqttPacket {
    fix_header: BytesMut;
    variable_header: BytesMut;
    payload: BytesMut;
}


pub struct Mqttcodec

impl Decoder for Mqttcodec{
    type Item = MqttPacket;
    type Error = io::Error;
    

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
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

        let fix_header = FixHeader::new(packet_type, dup_flag, qos_level, retain, remaining_len);

        src.split_to(remaining_len)


    }
}



fn main() {
    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let (tx, rx) = channel();

    let socket_addr = "127.0.0.1".parse();
    let fut = TcpStream::connect(&socket_addr, &handle);

    let client = fut.and_then(|stream| {
        
    });
    
    


}