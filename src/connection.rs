use openssl::ssl::*;
use protobuf::CodedOutputStream;

use std::io::{Cursor, Read};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;

use crate::message_types::MessageType::*;
use crate::message_types::*;
use crate::mumble;

/// A struct responsible for sending and receiving messages from the Mumble Server.
pub struct Connection {
    stream: SslStream<TcpStream>,
}

impl Connection {
    /// Create a new connection to a Mumble server.
    pub fn new(
        url: &str,
        key: &Path,
        cert: &Path,
        username: &str,
        password: Option<&str>,
    ) -> Connection {
        // Setup TCP stream
        let ctx = Connection::setup_tls(key, cert).expect("Failed to setup TLS");
        let tcp = TcpStream::connect(url).expect("Invalid URL");
        tcp.set_read_timeout(Some(std::time::Duration::from_secs(1)))
            .unwrap();
        tcp.set_write_timeout(Some(std::time::Duration::from_secs(1)))
            .unwrap();
        let stream = Ssl::new(&ctx)
            .expect("Failed to construct Ssl")
            .connect(tcp)
            .expect("Failed to connect Ssl to the TCP stream");

        let mut connection = Self { stream };

        connection.initialize(username, password);
        connection
    }

    /// Initialize the connection by sending the necessary information to the Mumble server.
    fn initialize(&mut self, username: &str, password: Option<&str>) {
        // Send version message
        let version_message = mumble::Version::new();
        self.send(&Version(version_message));

        // Send authentication message
        let mut authenticate_message = mumble::Authenticate::new();
        authenticate_message.set_username(username.to_string());
        authenticate_message.set_password(password.unwrap_or_default().to_string());
        self.send(&Authenticate(authenticate_message));
    }

    /// Returns an SslContext that is setup with the supplied key and certificate.
    fn setup_tls(key: &Path, cert: &Path) -> Result<Arc<SslContext>, openssl::error::ErrorStack> {
        let mut ctx = SslContext::builder(SslMethod::tls())?;
        ctx.set_verify(SslVerifyMode::NONE);
        ctx.set_certificate_chain_file(cert)?;
        ctx.set_private_key_file(&key, SslFiletype::PEM)?;
        ctx.set_cipher_list("AES128-SHA:AES256-SHA")?;
        Ok(Arc::new(ctx.build()))
    }

    /// Returns a message received from the Mumble server if there is one.
    pub fn read(&mut self) -> Option<MessageType> {
        // The first two bytes of a message represent the id/type of the message
        let mut id_bytes = [0; 2];
        self.stream.read_exact(&mut id_bytes).ok()?;

        let id = u16::from_le_bytes(id_bytes);

        // The next four bytes specify the length of the message
        let mut length_bytes = [0; 4];
        self.stream.read_exact(&mut length_bytes).ok()?;

        let length = u32::from_le_bytes(length_bytes);

        // The rest of the message is the payload (length bytes)
        let mut payload = vec![0; length as usize];
        self.stream.read_exact(&mut payload).ok()?;

        // Evaluate payload
        let message = MessageType::from_raw(id, &payload).ok()?;

        Some(message)
    }

    /// Sends a message to the Mumble server.
    pub fn send(&mut self, message: &MessageType) {
        // Create a stream to output the message
        let mut output_stream = CodedOutputStream::new(&mut self.stream);

        // Send bytes
        output_stream
            .write_raw_bytes(&message.to_raw())
            .expect("Could not write bytes");
        output_stream.flush().expect("Could not flush bytes");
    }
}
