use std::net::TcpStream;
use openssl::ssl::*;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use openssl::x509::X509FileType;

mod proto;

pub struct Connection {
    stream: SslStream<TcpStream>,
    url: String,
    key: PathBuf,
    cert: PathBuf
}

impl Connection {
    fn setup_tls(key: &Path, cert: &Path) -> Arc<SslContext> {
        let mut ctx = SslContext::new(SslMethod::Tlsv1).unwrap();
        ctx.set_verify(SSL_VERIFY_NONE, None);
        ctx.set_certificate_chain_file(cert, X509FileType::PEM).unwrap();
        ctx.set_private_key_file(&key, X509FileType::PEM).unwrap();
        ctx.set_cipher_list("ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:AES128-GCM-SHA256:AES256-GCM-SHA384:AES128-SHA256:AES256-SHA256:AES128-SHA:AES256-SHA:ECDHE-ECDSA-DES-CBC3-SHA:ECDHE-RSA-DES-CBC3-SHA:DES-CBC3-SHA:!aNULL:!eNULL:!EXPORT:!DES:!RC4:!MD5:!PSK:!aECDH:!EDH-DSS-DES-CBC3-SHA:!EDH-RSA-DES-CBC3-SHA:!KRB5-DES-CBC3-SHA").unwrap();
        return Arc::new(ctx);
    }

    pub fn new(url: &str, key: &Path, cert: &Path) -> Connection {
        let ctx = Connection::setup_tls(key, cert);
        let tcp = TcpStream::connect(url).unwrap();
        let stream = SslStream::connect(&*ctx, tcp).unwrap();
        return Connection {
            stream: stream,
            url: String::from(url),
            key: PathBuf::from(key),
            cert: PathBuf::from(cert)
        };
    }

    fn send(&mut self, message: &proto::Message) {
        self.stream.write(message.to_buffer().as_slice());
    }

    pub fn send_version(&mut self) {
        self.send(&proto::Version::new(proto::SemVer::new(1, 2, 7), "Rumble", "Unknown", "Unknown"));
    }
}
