extern crate rumble;

use rumble::{connect};
use std::path::Path;

fn main() {
    connect("localhost:64738", Path::new("bot.key"), Path::new("bot.crt"));
    loop {

    }
}
