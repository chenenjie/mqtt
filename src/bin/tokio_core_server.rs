
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_service;
extern crate tokio_core;


use std::io;
use std::str;
use bytes::BytesMut;
use futures::{future, Future, BoxFuture, Stream, Sink};
use tokio_io::codec::{Framed, Encoder, Decoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_service::{Service, NewService};




pub struct LineCodec;


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
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}


impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

struct EchoService;

impl Service for EchoService{
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<String, io::Error>;
    
    fn call(&self, input: String) -> Self::Future{
        future::ok(input).boxed()
    }
}

fn serve<S>(s: S) -> io::Result<()>
    where S: NewService<Request = String,
                        Response = String,
                        Error = io::Error> + 'static {

    let mut core = Core::new()?;
    let handle = core.handle();

    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &handle)?;


    let connections = listener.incoming();
    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.framed(LineCodec).split();
        let service = s.new_service()?;

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses)
            .then(|_| Ok(()));
        handle.spawn(server);
        Ok(())
    });

    core.run(server)
}


fn main(){
    if let Err(e) = serve(|| Ok(EchoService)) {
        println!("Server failed with {}", e);
    }
}
                                                        
