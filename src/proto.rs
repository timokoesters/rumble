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

/// # Protocol: Version Information
///
/// Stores the version information transmitted from or to the server.
///
pub struct ProtoVersion {
    /// Encoded version of the mumble protocol the client/server is supporting.
    pub version: u32,
    /// Exact release name of the software the other side is using.
    pub release: String,
    /// Name of the operating system the client/server is running on.
    pub os: String,
    /// Specific version of the operating system.
    pub os_version: String
}

impl ProtoVersion {
    fn encode_version(version: SemVer) -> u32 {
        return ((version.major as u32 & 0xFFFF) << 16) |
            ((version.minor as u32 & 0xFF) << 8) |
            (version.patch as u32 & 0xFF);
    }

    pub fn set_semver(&mut self, version: SemVer) {
        let encoded = ProtoVersion::encode_version(version);
        self.version = encoded;
    }
}
