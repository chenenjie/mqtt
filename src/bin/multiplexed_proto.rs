extern crate tokio_service;
extern crate tokio_core;
extern crate tokio_proto;
extern crate bytes;
extern crate tokio_io;
extern crate futures;

use std::io;
use std::str;

use tokio_proto::multiplex::RequestId;
use bytes::{Buf, BigEndian, BytesMut, BufMut, IntoBuf};
use tokio_proto::multiplex::ServerProto;
use tokio_io::codec::{Framed, Encoder, Decoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;

struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;

    // `Framed<T, LineCodec>` is the return value
    // of `io.framed(LineCodec)`
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

struct LineCodec;

impl Decoder for LineCodec {
    type Item = (RequestId, String);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut)
             -> io::Result<Option<(RequestId, String)>>
    {
        // At least 5 bytes are required for a frame: 4 byte
        // head + one byte '\n'
        if buf.len() < 5 {
            // We don't yet have a full message
            return Ok(None);
        }

        // Check to see if the frame contains a new line, skipping
        // the first 4 bytes which is the request ID
        let newline = buf[4..].iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            // remove the serialized frame from the buffer.
            let mut line = buf.split_to(n + 4);

            // Also remove the '\n'
            buf.split_to(1);

            // Deserialize the request ID
            let id = line.split_to(4).into_buf().get_u32::<BigEndian>();

            // Turn this data into a UTF string and return it in a Frame.
            return match str::from_utf8(&line[4..]) {
                Ok(s) => Ok(Some((id as RequestId, s.to_string()))),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid string")),
            }
        }

        // No `\n` found, so we don't have a complete message
        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = (RequestId, String);
    type Error = io::Error;

    fn encode(&mut self,
              msg: (RequestId, String),
              buf: &mut BytesMut) -> io::Result<()>
    {
        let (id, msg) = msg;

        buf.put_u32::<BigEndian>(id as u32);
        buf.put(msg.as_bytes());
        buf.put("\n");

        Ok(())
    }
}

struct Echo;

impl Service for Echo {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}
