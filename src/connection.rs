use byteorder::{BigEndian, ReadBytesExt};
use openssl::ssl::*;
use protobuf::CodedOutputStream;
use std::io::{Cursor, Read};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use message_types::MessageType::*;
use message_types::*;
use mumble;

// Connection timeout in seconds. It waits this long if no message was received
const TIMEOUT: u64 = 5;

// How long to wait between pings
const PING_DELAY: u64 = 10;

/// A way to interact with the Mumble server
pub struct Connection {
    stream: SslStream<TcpStream>,
    session: Option<u32>,
    users: Vec<mumble::UserState>,
    // TODO: channels: Vec<mumble::ChannelState>,
    last_ping: u64,
}

impl Connection {
    pub fn new(url: &str, key: &Path, cert: &Path) -> Connection {
        // Setup TCP stream
        let ctx = Connection::setup_tls(key, cert);
        let tcp = TcpStream::connect(url).unwrap();
        tcp.set_read_timeout(Some(Duration::new(TIMEOUT, 0)))
            .unwrap();
        let stream = Ssl::new(&ctx).unwrap().connect(tcp).unwrap();

        Self {
            stream,
            session: None,
            users: vec![],
            //channels: vec![],
            last_ping: 0,
        }
    }

    fn setup_tls(key: &Path, cert: &Path) -> Arc<SslContext> {
        let mut ctx = SslContext::builder(SslMethod::tls()).unwrap();
        ctx.set_verify(SslVerifyMode::NONE);
        ctx.set_certificate_chain_file(cert).unwrap();
        ctx.set_private_key_file(&key, SslFiletype::PEM).unwrap();
        ctx.set_cipher_list("ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:AES128-GCM-SHA256:AES256-GCM-SHA384:AES128-SHA256:AES256-SHA256:AES128-SHA:AES256-SHA:ECDHE-ECDSA-DES-CBC3-SHA:ECDHE-RSA-DES-CBC3-SHA:DES-CBC3-SHA:!aNULL:!eNULL:!EXPORT:!DES:!RC4:!MD5:!PSK:!aECDH:!EDH-DSS-DES-CBC3-SHA:!EDH-RSA-DES-CBC3-SHA:!KRB5-DES-CBC3-SHA").unwrap();
        Arc::new(ctx.build())
    }

    pub fn read(&mut self) -> Result<MessageType, std::io::Error> {
        // The first two bytes of a message represent the id/type of the message
        let mut id_bytes = [0; 2];
        self.stream
            .ssl_read(&mut id_bytes)
            .map_err(|e| e.into_io_error().unwrap())?;

        let id = Cursor::new(id_bytes).read_u16::<BigEndian>().unwrap();

        // The next four bytes specify the length of the message
        let mut length_bytes = [0; 4];
        self.stream
            .ssl_read(&mut length_bytes)
            .map_err(|e| e.into_io_error().unwrap())?;

        let length = Cursor::new(length_bytes).read_u32::<BigEndian>().unwrap();

        // The rest of the message is the payload (length bytes)
        let mut payload = vec![0; length as usize];
        self.stream.read_exact(&mut payload)?;

        // Evaluate payload
        let message = MessageType::from_raw(id, &payload);

        // Handle special messages
        match &message {
            // Save the current session
            ServerSync(data) => self.session = Some(data.get_session()),

            // Keep a list of all users
            UserState(data) => {
                // Replace or append user
                if let Some(index) = self
                    .users
                    .iter()
                    .position(|u| u.get_session() == data.get_session())
                {
                    // If there is a new channel id we update it
                    if data.has_channel_id() {
                        self.users[index].set_channel_id(data.get_channel_id());
                    }
                } else {
                    // Add user
                    self.users.push(*data.clone());
                }
            }
            // Ignore other messages
            _ => {}
        };

        Ok(message)
    }

    pub fn send(&mut self, message: &MessageType) {
        // Create a stream to output the message
        let mut output_stream = CodedOutputStream::new(&mut self.stream);

        // Send bytes
        output_stream.write_raw_bytes(&message.to_raw()).unwrap();
        output_stream.flush().unwrap();
    }

    pub fn send_version(&mut self) {
        // Send version message
        let message = mumble::Version::new();
        self.send(&Version(message));
    }

    pub fn send_authentication(&mut self, username: &str, password: &str) {
        // Send authentication message
        let mut message = mumble::Authenticate::new();
        message.set_username(username.to_string());
        message.set_password(password.to_string());
        self.send(&Authenticate(message));
    }

    pub fn send_text_message(&mut self, text: &str) {
        // Send text message
        match self.session {
            Some(s) => {
                let mut message = mumble::TextMessage::new();
                message.set_actor(s);
                message.set_channel_id(vec![0]);
                message.set_message(text.to_string());
                self.send(&TextMessage(message));
            }
            None => println!("Connection not established. (No session)"),
        }
    }

    pub fn change_channel(&mut self, channel_id: u32) {
        // Send channel change message
        if let Some(s) = self.session {
            let mut message = mumble::UserState::new();
            message.set_session(s);
            message.set_actor(s);
            message.set_channel_id(channel_id);
            self.send(&UserState(Box::new(message)));
        } else {
            println!("Connection not established. (No session)");
        }
    }

    pub fn keep_alive(&mut self) {
        // Send ping message if necessary
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Return if no new ping is necessary
        if timestamp - self.last_ping < PING_DELAY {
            return;
        }

        // Send ping
        let mut ping = mumble::Ping::new();
        ping.set_timestamp(timestamp);
        self.last_ping = timestamp;
        self.send(&Ping(ping));
    }

    pub fn get_users(&self) -> &Vec<mumble::UserState> {
        &self.users
    }
}

pub fn connect(url: &str, key: &Path, cert: &Path, username: &str, password: &str) -> Connection {
    let mut connection = Connection::new(url, key, cert);
    connection.send_version();
    connection.send_authentication(username, password);
    connection
}
