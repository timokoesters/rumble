extern crate protobuf;

mod mumble;

// use std::io::prelude::*;
use std::net::TcpStream;
use std::string::String;
use protobuf::Message;

fn encode_version(major: u16, minor: u8, patch: u8) -> u32 {
    return ((major as u32 & 0xFFFF) << 16) |
        ((minor as u32 & 0xFF) << 8) |
        (patch as u32 & 0xFF);
}

pub fn connect(url: &str) {
    let mut stream = TcpStream::connect(url).unwrap();
    let mut version = mumble::Version::new();

    let mut proto_stream = protobuf::CodedOutputStream::new(&mut stream);
    version.set_version(encode_version(1, 2, 7));
    version.set_release(String::from("Rumble"));
    let _ = version.write_to_with_cached_sizes(&mut proto_stream);
}
