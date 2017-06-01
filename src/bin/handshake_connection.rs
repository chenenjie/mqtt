extern crate tokio_io;
extern crate tokio_proto;
extern crate bytes;
extern crate futures;

use tokio_io::{AsyncRead, AsyncWrite};
use bytes::BytesMut;
use tokio_io::codec::{Framed, Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;
use futures::{future, Stream, Future, Sink};
use std::io;

struct LineProto;

struct LineCodec;

impl Encoder for LineCodec {
  type Item = String;
  type Error = io::Error;

  fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
      buf.extend(msg.as_bytes());
      buf.extend(b"\n");
      Ok(())
  }
}
impl Decoder for LineCodec {
  type Item = String;
  type Error = io::Error;

  fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
      if let Some(i) = buf.iter().position(|&b| b == b'\n') {
          // remove the serialized frame from the buffer.
          let line = buf.split_to(i);

          // Also remove the '\n'
          buf.split_to(1);

          // Turn this data into a UTF string and return it in a Frame.
          match std::str::from_utf8(&line) {
              Ok(s) => Ok(Some(s.to_string())),
              Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                           "invalid UTF-8")),
          }
      } else {
          Ok(None)
      }
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto
{
    type Request = String;
    type Response = String;

    // `Framed<T, LineCodec>` is the return value of
    // `io.framed(LineCodec)`
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Box<Future<Item = Self::Transport,
                                   Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        // Construct the line-based transport
        let transport = io.framed(LineCodec);

        // The handshake requires that the client sends `You ready?`,
        // so wait to receive that line. If anything else is sent,
        // error out the connection
        Box::new(transport.into_future()
            // If the transport errors out, we don't care about
            // the transport anymore, so just keep the error
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                // A line has been received, check to see if it
                // is the handshake
                match line {
                    Some(ref msg) if msg == "You ready?" => {
                        println!("SERVER: received client handshake");
                        // Send back the acknowledgement
                        let ret = transport.send("Bring it!".into());
                        Box::new(ret) as Self::BindTransport
                    }
                    _ => {
                        // The client sent an unexpected handshake,
                        // error out the connection
                        println!("SERVER: client handshake INVALID");
                        let err = io::Error::new(io::ErrorKind::Other,
                                                 "invalid handshake");
                        let ret = future::err(err);
                        Box::new(ret) as Self::BindTransport
                    }
                }
            }))
    }
}
fn main() {}
