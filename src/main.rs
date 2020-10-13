use std::os::unix::net::UnixStream;
use protobuf::Message;

mod aesm_proto;

fn main() -> std::io::Result<()> {

    let req = aesm_proto::Request::new();
    let init = req.get_initQuoteReq();

    let mut stream = UnixStream::connect("/var/run/aesmd/aesm.socket")?; // this connects to aesmd on the host
    init.write_to_writer(&mut stream)?;
    Ok(())
}
