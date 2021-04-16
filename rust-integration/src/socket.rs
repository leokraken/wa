use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let mut stream = UnixStream::connect("/Users/lclavijo/node/testapi/app.socket")?;
  stream.write_all(b"GET /ping HTTP/1.0\r\n\r\n")?;
  let mut response = String::new();
  stream.read_to_string(&mut response)?;
  println!("{}", response);
  Ok(())
}