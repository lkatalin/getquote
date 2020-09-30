use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use prost::Message;

mod aesm;

fn main() -> std::io::Result<()> {
    //let req = aesm::request::InitQuoteRequest { timeout: None }; // malformed request received
    let req = aesm::Request { init_quote_req: Some(aesm::request::InitQuoteRequest { timeout: Some(10) } ) };
    let mut buf = vec![];
    req.encode(&mut buf).unwrap();

    let mut stream = UnixStream::connect("/var/run/aesmd/aesm.socket")?;
    stream.write_all(&buf)?;
    stream.read_exact(&mut buf)?;
    println!("{:?}", buf);
    Ok(())
}
