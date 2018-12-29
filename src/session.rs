use crate::connection::Connection;
use crate::message_types::MessageType;
use crate::message_types::MessageType::*;
use crate::mumble;
use std::path::Path;

/// A struct that is able to connect to a Mumble server and interact with it.
pub struct Session {
    pub connection: Connection,
    pub session_id: u32,
    pub channel_id: u32,
    users: Vec<mumble::UserState>,
    channels: Vec<mumble::ChannelState>,
    last_ping: std::time::Instant,
}

impl Session {
    /// Returns a session that is connected to a Mumble server.
    pub fn connect(
        url: &str,
        key: &Path,
        cert: &Path,
        username: &str,
        password: Option<&str>,
    ) -> Session {
        let mut session = Self {
            connection: Connection::new(url, key, cert, username, password),
            session_id: 0,
            channel_id: 0,
            users: vec![],
            channels: vec![],
            last_ping: std::time::Instant::now(),
        };

        // Synchronize with the server
        loop {
            if let Some(message) = session.connection.read() {
                match message {
                    ServerSync(data) => {
                        session.session_id = data.get_session();
                        // The ServerSync message is the last message of the synchronization
                        break;
                    }

                    // Keep a list of all users
                    UserState(data) => {
                        // Add user
                        session.users.push(*data.clone());
                    }

                    // Ignore other messages
                    _ => {}
                }
            }
        }
        session
    }

    /// Handles the message received from the Mumble server and returns it if there is one.
    pub fn read(&mut self) -> Option<MessageType> {
        let message = self.connection.read()?;
        match &message {
            UserState(data) => {
                // Update or append user
                if let Some(index) = self
                    .users
                    .iter()
                    .position(|u| u.get_session() == data.get_session())
                {
                    // If there is a new channel we update it
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
        }

        Some(message)
    }

    /// Sends a text message into the chat of the current channel.
    pub fn send_text_message(&mut self, text: &str) {
        self.send_text_message_to(text, self.channel_id);
    }

    pub fn send_text_message_to(&mut self, text: &str, channel: u32) {
        // Send text message
        let mut message = mumble::TextMessage::new();
        message.set_actor(self.session_id);
        message.set_channel_id(vec![channel]);
        message.set_message(text.to_string());
        self.connection.send(&TextMessage(message));
    }

    /// Switches the channel to another one.
    pub fn change_channel(&mut self, channel_id: u32) {
        // Send channel change message
        let mut message = mumble::UserState::new();
        message.set_session(self.session_id);
        message.set_actor(self.session_id);
        message.set_channel_id(channel_id);
        self.connection.send(&UserState(Box::new(message)));

        self.channel_id = channel_id;
    }

    /// Pings the Server if it is necessary. You need to call this regularly.
    pub fn keep_alive(&mut self) {
        // Send ping message if necessary
        let delay = self.last_ping.elapsed();

        // Return if no new ping is necessary
        if delay < std::time::Duration::from_secs(20) {
            return;
        }

        // Send ping
        let ping = mumble::Ping::new();
        self.last_ping = std::time::Instant::now();
        self.connection.send(&Ping(ping));
    }

    /// Returns a vector of all other users that are connected to the server.
    pub fn get_users(&self) -> &Vec<mumble::UserState> {
        &self.users
    }
}
