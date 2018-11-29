use mumble;
use protobuf::{CodedInputStream, Message, ProtobufResult};

#[derive(Debug)]
pub enum MessageType {
    Version(mumble::Version),
    UDPTunnel,
    Authenticate(mumble::Authenticate),
    Ping(mumble::Ping),
    Reject(mumble::Reject),
    ServerSync(mumble::ServerSync),
    ChannelRemove(mumble::ChannelRemove),
    ChannelState(Box<mumble::ChannelState>),
    UserRemove(mumble::UserRemove),
    UserState(Box<mumble::UserState>),
    BanList(mumble::BanList),
    TextMessage(mumble::TextMessage),
    PermissionDenied(mumble::PermissionDenied),
    ACL(mumble::ACL),
    QueryUsers(mumble::QueryUsers),
    CryptSetup(mumble::CryptSetup),
    ContextActionModify(mumble::ContextActionModify),
    ContextAction(mumble::ContextAction),
    UserList(mumble::UserList),
    VoiceTarget(mumble::VoiceTarget),
    PermissionQuery(mumble::PermissionQuery),
    CodecVersion(mumble::CodecVersion),
    UserStats(Box<mumble::UserStats>),
    RequestBlob(mumble::RequestBlob),
    ServerConfig(mumble::ServerConfig),
    SuggestConfig(mumble::SuggestConfig),
}

impl MessageType {
    pub fn interpret_message<M: Message>(payload: &[u8]) -> ProtobufResult<M> {
        let mut message = M::new();
        M::merge_from(&mut message, &mut CodedInputStream::from_bytes(payload))?;
        Ok(message)
    }

    pub fn from_raw(id: u16, payload: &[u8]) -> MessageType {
        use message_types::MessageType::*;
        match id {
            0 => Version(Self::interpret_message::<mumble::Version>(&payload).unwrap()),
            1 => UDPTunnel,
            2 => Authenticate(Self::interpret_message::<mumble::Authenticate>(&payload).unwrap()),
            3 => Ping(Self::interpret_message::<mumble::Ping>(&payload).unwrap()),
            4 => Reject(Self::interpret_message::<mumble::Reject>(&payload).unwrap()),
            5 => ServerSync(Self::interpret_message::<mumble::ServerSync>(&payload).unwrap()),
            6 => ChannelRemove(Self::interpret_message::<mumble::ChannelRemove>(&payload).unwrap()),
            7 => ChannelState(Box::new(
                Self::interpret_message::<mumble::ChannelState>(&payload).unwrap(),
            )),
            8 => UserRemove(Self::interpret_message::<mumble::UserRemove>(&payload).unwrap()),
            9 => UserState(Box::new(
                Self::interpret_message::<mumble::UserState>(&payload).unwrap(),
            )),
            10 => BanList(Self::interpret_message::<mumble::BanList>(&payload).unwrap()),
            11 => TextMessage(Self::interpret_message::<mumble::TextMessage>(&payload).unwrap()),
            12 => PermissionDenied(
                Self::interpret_message::<mumble::PermissionDenied>(&payload).unwrap(),
            ),
            13 => ACL(Self::interpret_message::<mumble::ACL>(&payload).unwrap()),
            14 => QueryUsers(Self::interpret_message::<mumble::QueryUsers>(&payload).unwrap()),
            15 => CryptSetup(Self::interpret_message::<mumble::CryptSetup>(&payload).unwrap()),
            16 => ContextActionModify(
                Self::interpret_message::<mumble::ContextActionModify>(&payload).unwrap(),
            ),
            17 => {
                ContextAction(Self::interpret_message::<mumble::ContextAction>(&payload).unwrap())
            }
            18 => UserList(Self::interpret_message::<mumble::UserList>(&payload).unwrap()),
            19 => VoiceTarget(Self::interpret_message::<mumble::VoiceTarget>(&payload).unwrap()),
            20 => PermissionQuery(
                Self::interpret_message::<mumble::PermissionQuery>(&payload).unwrap(),
            ),
            21 => CodecVersion(Self::interpret_message::<mumble::CodecVersion>(&payload).unwrap()),
            22 => UserStats(Box::new(
                Self::interpret_message::<mumble::UserStats>(&payload).unwrap(),
            )),
            23 => RequestBlob(Self::interpret_message::<mumble::RequestBlob>(&payload).unwrap()),
            24 => ServerConfig(Self::interpret_message::<mumble::ServerConfig>(&payload).unwrap()),
            25 => {
                SuggestConfig(Self::interpret_message::<mumble::SuggestConfig>(&payload).unwrap())
            }
            _ => unreachable!(),
        }
    }
}

pub trait HasType {
    fn get_id(&self) -> u16;
}

impl HasType for mumble::Version {
    fn get_id(&self) -> u16 {
        0
    }
}

impl HasType for mumble::UDPTunnel {
    fn get_id(&self) -> u16 {
        1
    }
}

impl HasType for mumble::Authenticate {
    fn get_id(&self) -> u16 {
        2
    }
}

impl HasType for mumble::Ping {
    fn get_id(&self) -> u16 {
        3
    }
}

impl HasType for mumble::Reject {
    fn get_id(&self) -> u16 {
        4
    }
}

impl HasType for mumble::ServerSync {
    fn get_id(&self) -> u16 {
        5
    }
}

impl HasType for mumble::ChannelRemove {
    fn get_id(&self) -> u16 {
        6
    }
}

impl HasType for mumble::ChannelState {
    fn get_id(&self) -> u16 {
        7
    }
}

impl HasType for mumble::UserRemove {
    fn get_id(&self) -> u16 {
        8
    }
}

impl HasType for mumble::UserState {
    fn get_id(&self) -> u16 {
        9
    }
}

impl HasType for mumble::BanList {
    fn get_id(&self) -> u16 {
        10
    }
}

impl HasType for mumble::TextMessage {
    fn get_id(&self) -> u16 {
        11
    }
}

impl HasType for mumble::PermissionDenied {
    fn get_id(&self) -> u16 {
        12
    }
}

impl HasType for mumble::ACL {
    fn get_id(&self) -> u16 {
        13
    }
}

impl HasType for mumble::QueryUsers {
    fn get_id(&self) -> u16 {
        14
    }
}

impl HasType for mumble::CryptSetup {
    fn get_id(&self) -> u16 {
        15
    }
}

impl HasType for mumble::ContextActionModify {
    fn get_id(&self) -> u16 {
        16
    }
}

impl HasType for mumble::ContextAction {
    fn get_id(&self) -> u16 {
        17
    }
}

impl HasType for mumble::UserList {
    fn get_id(&self) -> u16 {
        18
    }
}

impl HasType for mumble::VoiceTarget {
    fn get_id(&self) -> u16 {
        19
    }
}

impl HasType for mumble::PermissionQuery {
    fn get_id(&self) -> u16 {
        20
    }
}

impl HasType for mumble::CodecVersion {
    fn get_id(&self) -> u16 {
        21
    }
}

impl HasType for mumble::UserStats {
    fn get_id(&self) -> u16 {
        22
    }
}

impl HasType for mumble::RequestBlob {
    fn get_id(&self) -> u16 {
        23
    }
}

impl HasType for mumble::ServerConfig {
    fn get_id(&self) -> u16 {
        24
    }
}

impl HasType for mumble::SuggestConfig {
    fn get_id(&self) -> u16 {
        25
    }
}
