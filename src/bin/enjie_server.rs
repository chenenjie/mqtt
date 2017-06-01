
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;
extern crate futures;

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;
use tokio_io::codec::{Framed, Encoder, Decoder};
use tokio_core::io::{read_exact, write_all, Window};
use std::io::{self, Error, ErrorKind};
use futures::{Future, Stream, Sink};

//struct LineCodec;

//impl Encoder for LineCodec {

    //type Item = BytesMut;
    //type Error = io::Error;

    //fn encode(&mut self, item: Self::Item, dst: &mut BytesMut)
        //-> Result<(), Self::Error>
    //{

    //}
//}

//impl Decoder for LineCodec {
    //type Item = BytesMut;

    //type Error = io::Error;

    //fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        
    //}
//}



fn serve() -> io::Result<()> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &handle)?;


    let connections = listener.incoming();
    let server = connections.for_each(move |(stream, _address)|{
        let fut = read_exact(stream, vec![0u8; 22]).and_then(|(conn, buf)|{
            if let Ok(req) = String::from_utf8(buf) {
                println!("{:?}", req);
                if req == "how about your father?" {
                    return Ok(conn);
                }
            }     
            Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
        });

        let fut = fut.and_then(|conn| {
            let response = String::from("good very good");
            //write_all(conn, response.into_bytes()).map(|(conn, buff)| (conn, buff))
            //write_all(conn, response.into_bytes()).and_then(|(conn, buff)| Ok((conn, buff)))
            //write_all(conn, response.into_bytes()).then(|result| {
                //result
            //})

            write_all(conn, response.into_bytes()).and_then(|(conn, _buf)| {
                read_exact(conn, vec![0u8; 22]).and_then(|(conn, buf)|{
                    if let Ok(req) = String::from_utf8(buf) {
                        if req == "how about your mother?" {
                            return Ok(conn);
                        }
                    }     
                    Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
                })
            })
        });

        //let fut = fut.and_then(|(conn, _buf)| {
            //read_exact(conn, vec![0u8; 22])        
        //});
        
        //let fut = fut.and_then(|(conn, buf)|{
            //if let Ok(req) = String::from_utf8(buf) {
                //if req == "how about your mother?" {
                    //return Ok(conn);
                //}
            //}     
            //Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
        //});

        let fut = fut.and_then(|conn| {
            let response = String::from("good very good");
            write_all(conn, response.into_bytes())
        }).then(|_| Ok(()));

        handle.spawn(fut);
        Ok(())
    });

    core.run(server)
}


fn main() {
    serve();
}
