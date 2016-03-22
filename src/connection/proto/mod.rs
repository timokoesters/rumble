mod convert;

use std::string::String;
use std::vec::Vec;


/// # Message
///
/// Represents a single Message
///
pub trait Message {
    fn to_buffer(&self) -> Vec<u8>;
}

/// # Message Type
///
/// Used to get the different message types of the mumble protocol into a readable format.
pub enum MessageType {
    Version = 0,
    Auth = 2,
    ChannelState = 7,
    CryptSetup = 15,
    ServerConfig = 24
}

/// # Semantic Version
///
/// Stores the a parsed semantic version.
///
pub struct SemVer {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u8,
    /// Patchlevel.
    pub patch: u8
}

impl SemVer {
    pub fn new(major: u16, minor: u8, patch: u8) -> SemVer {
        return SemVer {
            major: major,
            minor: minor,
            patch: patch
        };
    }
}

/// # Protocol: Version Information
///
/// Stores the version information transmitted from or to the server.
///
#[derive(Clone)]
pub struct Version {
    /// Encoded version of the mumble protocol the client/server is supporting.
    pub version: u32,
    /// Exact release name of the software the other side is using.
    pub release: String,
    /// Name of the operating system the client/server is running on.
    pub os: String,
    /// Specific version of the operating system.
    pub os_version: String
}

impl Version {
    pub fn new(version: SemVer, release: &str, os: &str, os_version: &str) -> Version {
        return Version {
            version: Version::encode_version(version),
            release: String::from(release),
            os: String::from(os),
            os_version: String::from(os_version)
        };
    }

    fn encode_version(version: SemVer) -> u32 {
        return ((version.major as u32 & 0xFFFF) << 16) |
            ((version.minor as u32 & 0xFF) << 8) |
            (version.patch as u32 & 0xFF);
    }
}

impl Message for Version {
    fn to_buffer(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend(convert::le_u32_to_u8(self.version).iter());
        vec.extend(self.release.as_bytes());
        vec.extend(self.os.as_bytes());
        vec.extend(self.os_version.as_bytes());
        return vec;
    }
}
