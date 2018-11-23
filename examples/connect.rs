extern crate rumble;

use rumble::{connect};
use std::path::Path;

fn main() {
    let mut connection = connect("localhost:64738", Path::new("bot.key"), Path::new("bot.crt"));
    connection.send_version();
    loop {
        connection.read();
    }
}
