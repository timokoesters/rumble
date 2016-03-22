extern crate openssl;

mod connection;

use std::path::Path;

pub fn connect(url: &str, key: &Path, cert: &Path) -> connection::Connection {
    let mut connection = connection::Connection::new(url, key, cert);
    connection.send_version();
    return connection;
}
