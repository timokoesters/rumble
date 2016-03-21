extern crate rumble;

use rumble::{ connect };

fn main() {
    connect("localhost:64738", "bot.key", "bot.crt");
}
