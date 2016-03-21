extern crate protobuf;
extern crate openssl;

mod mumble;

use std::string::String;
use protobuf::{Message, CodedOutputStream, CodedInputStream};
use std::net::TcpStream;
use openssl::ssl::*;
use std::sync::Arc;
use std::path::Path;
use openssl::x509::X509FileType;

fn encode_version(major: u16, minor: u8, patch: u8) -> u32 {
    return ((major as u32 & 0xFFFF) << 16) |
        ((minor as u32 & 0xFF) << 8) |
        (patch as u32 & 0xFF);
}

fn setup_tls(key: &str, cert: &str) -> Arc<SslContext> {
    let mut ctx = SslContext::new(SslMethod::Tlsv1).unwrap();
    ctx.set_verify(SSL_VERIFY_NONE, None);
    ctx.set_certificate_chain_file(&Path::new(&cert), X509FileType::PEM).unwrap();
    ctx.set_private_key_file(&Path::new(&key), X509FileType::PEM).unwrap();
    ctx.set_cipher_list("ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:AES128-GCM-SHA256:AES256-GCM-SHA384:AES128-SHA256:AES256-SHA256:AES128-SHA:AES256-SHA:ECDHE-ECDSA-DES-CBC3-SHA:ECDHE-RSA-DES-CBC3-SHA:DES-CBC3-SHA:!aNULL:!eNULL:!EXPORT:!DES:!RC4:!MD5:!PSK:!aECDH:!EDH-DSS-DES-CBC3-SHA:!EDH-RSA-DES-CBC3-SHA:!KRB5-DES-CBC3-SHA").unwrap();
    return Arc::new(ctx);
}

fn send_version(output_stream: &mut CodedOutputStream) {
    let mut version = mumble::Version::new();
    version.set_version(encode_version(1, 2, 7));
    version.set_release(String::from("Rumble"));
    let result = version.write_to_with_cached_sizes(output_stream);
    println!("Sent Version, Status: {:?}", result);
}

fn send_auth(output_stream: &mut CodedOutputStream) {
    let mut authenticate = mumble::Authenticate::new();
    authenticate.set_username(String::from("RumbleTestbot"));
    let result = authenticate.write_to_with_cached_sizes(output_stream);
    println!("Sent Authenticate, Status: {:?}", result);
}

fn read_version(input_stream: &mut CodedInputStream) {
    let mut version = mumble::Version::new();
    let result = version.merge_from(input_stream);
    println!("Received Version, Status: {:?}, Data: {:?}", result, version);
}

fn read_crypt(input_stream: &mut CodedInputStream) {
    let mut crypt = mumble::CryptSetup::new();
    let result = crypt.merge_from(input_stream);
    println!("Received Crypt, Status: {:?}, Data: {:?}", result, crypt);
}

fn read_channelstate(input_stream: &mut CodedInputStream) {
    let mut channelstate = mumble::ChannelState::new();
    let result = channelstate.merge_from(input_stream);
    println!("Received ChannelState, Status: {:?}, Data: {:?}", result, channelstate);
}


pub fn connect(url: &str, key: &str, cert: &str) {
    let tcp = TcpStream::connect(url).unwrap();
    let ctx = setup_tls(key, cert);
    let mut stream1 = SslStream::connect(&*ctx, tcp).unwrap();
    let mut stream2 = stream1.try_clone().unwrap();
    let mut output_stream = protobuf::CodedOutputStream::new(&mut stream1);
    let mut input_stream = protobuf::CodedInputStream::new(&mut stream2);
    // Once the TLS handshake is completed both sides should transmit their version information
    // using the Version message.
    send_version(&mut output_stream);
    // Once the client has sent the version it should follow this with the Authenticate message.
    // This message may be sent immediately after sending the version message.
    // The client does not need to wait for the server version message.
    send_auth(&mut output_stream);
    read_version(&mut input_stream);
    // Once the Version packets are exchanged the server will send a CryptSetup packet to the client.
    read_crypt(&mut input_stream);

}
