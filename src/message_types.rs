use byteorder::{BigEndian, WriteBytesExt};
use message_types::MessageType::*;
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

    pub fn to_raw(&self) -> Vec<u8> {
        match self {
            Version(data) => Self::create_bytes(0, data),
            Authenticate(data) => Self::create_bytes(2, data),
            Ping(data) => Self::create_bytes(3, data),
            Reject(data) => Self::create_bytes(4, data),
            ServerSync(data) => Self::create_bytes(5, data),
            ChannelRemove(data) => Self::create_bytes(6, data),
            ChannelState(data) => Self::create_bytes(7, &**data),
            UserRemove(data) => Self::create_bytes(8, data),
            UserState(data) => Self::create_bytes(9, &**data),
            BanList(data) => Self::create_bytes(10, data),
            TextMessage(data) => Self::create_bytes(11, data),
            PermissionDenied(data) => Self::create_bytes(12, data),
            ACL(data) => Self::create_bytes(13, data),
            QueryUsers(data) => Self::create_bytes(14, data),
            CryptSetup(data) => Self::create_bytes(15, data),
            ContextActionModify(data) => Self::create_bytes(16, data),
            ContextAction(data) => Self::create_bytes(17, data),
            UserList(data) => Self::create_bytes(18, data),
            VoiceTarget(data) => Self::create_bytes(19, data),
            PermissionQuery(data) => Self::create_bytes(20, data),
            CodecVersion(data) => Self::create_bytes(21, data),
            UserStats(data) => Self::create_bytes(22, &**data),
            RequestBlob(data) => Self::create_bytes(23, data),
            ServerConfig(data) => Self::create_bytes(24, data),
            SuggestConfig(data) => Self::create_bytes(25, data),
            _ => panic!("Cannot send this type of data"),
        }
    }

    fn create_bytes<M: Message>(id: u16, data: &M) -> Vec<u8> {
        // Create vector of bytes to output
        let mut result = Vec::new();

        // Append id
        result.write_u16::<BigEndian>(id).unwrap();

        // Create payload from message
        let mut payload = Vec::new();
        data.write_to_vec(&mut payload).unwrap();

        // Append length
        result.write_u32::<BigEndian>(payload.len() as u32).unwrap();

        // Append payload
        result.append(&mut payload);

        result
    }
}
