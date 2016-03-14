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
    {
        let mut output_stream = protobuf::CodedOutputStream::new(&mut stream);
        let mut version = mumble::Version::new();
        version.set_version(encode_version(1, 2, 7));
        version.set_release(String::from("Rumble"));
        let result = version.write_to_with_cached_sizes(&mut output_stream);
        println!("Sent Version, Status: {:?}", result);
    }
    {
        let mut input_stream = protobuf::CodedInputStream::new(&mut stream);
        let mut version = mumble::Version::new();
        let result = version.merge_from(&mut input_stream);
        println!("Received Version, Status: {:?}, Data: {:?}", result, version);
    }
    {
        let mut output_stream = protobuf::CodedOutputStream::new(&mut stream);
        let mut authenticate = mumble::Authenticate::new();
        authenticate.set_username(String::from("RumbleTestbot"));
        let result = authenticate.write_to_with_cached_sizes(&mut output_stream);
        println!("Sent Authenticate, Status: {:?}", result);
    }
    {
        let mut input_stream = protobuf::CodedInputStream::new(&mut stream);
        let mut crypt = mumble::CryptSetup::new();
        let result = crypt.merge_from(&mut input_stream);
        println!("Received Crypto, Status: {:?}, Data: {:?}", result, crypt);
    }

}
