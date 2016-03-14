extern crate protobuf;

mod mumble;

use std::io::prelude::*;
use std::net::TcpStream;

pub fn connect(url: &str) {
    let mut stream = TcpStream::connect(url).unwrap();
    let _ = stream.read(&mut [0; 128]);
}
