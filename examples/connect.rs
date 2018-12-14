use rumble::session;
use rumble::message_types::MessageType::*;
use std::path::Path;

fn main() {
    let mut session = session::Session::connect(
        "localhost:64738",
        Path::new("bot.key"),
        Path::new("bot.crt"),
        "Bot",
        Some("password"),
    );

    loop {
        if let Some(message) = session.connection.read() {
            match message {
                // Echo messages
                TextMessage(data) => {
                    session.send_text_message(&format!("You said: {}", data.get_message()));
                }

                // Ignore some messages
                UDPTunnel | Ping(_) => {}

                // Print unhandled messages
                _ => println!("{:?}", message),
            }
        }
        session.keep_alive();
    }
}
