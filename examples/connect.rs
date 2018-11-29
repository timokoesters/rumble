extern crate rumble;

use rumble::connection;
use rumble::message_types::MessageType::*;
use std::path::Path;

fn main() {
    let mut connection = connection::connect(
        "localhost:64738",
        Path::new("bot.key"),
        Path::new("bot.crt"),
        "Bot",
        "password",
    );

    loop {
        if let Ok(message) = connection.read() {
            match message {
                // Echo messages
                TextMessage(data) => {
                    connection.send_text_message(&format!("You said: {}", data.get_message()));
                }

                // Ignore some messages
                UDPTunnel | Ping(_) => {}

                // Print unhandled messages
                _ => println!("{:?}", message),
            }
            connection.keep_alive();
        }
    }
}
