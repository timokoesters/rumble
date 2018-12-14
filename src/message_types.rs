use byteorder::{BigEndian, WriteBytesExt};
use crate::mumble;
use protobuf::{CodedInputStream, Message, ProtobufError, ProtobufResult};

/// An enum which contains all possible messages bundled together with it's content.
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
    /// Tries to interpret the supplied bytes as a message.
    fn interpret_bytes<M: Message>(payload: &[u8]) -> ProtobufResult<M> {
        let mut message = M::new();
        M::merge_from(&mut message, &mut CodedInputStream::from_bytes(payload))?;
        Ok(message)
    }

    /// Tries to interpret the supplied bytes as a message of type `id`.
    pub fn from_raw(id: u16, payload: &[u8]) -> ProtobufResult<MessageType> {
        use crate::message_types::MessageType::*;

        Ok(match id {
            0 => Version(Self::interpret_bytes::<mumble::Version>(&payload)?),
            1 => UDPTunnel,
            2 => Authenticate(Self::interpret_bytes::<mumble::Authenticate>(&payload)?),
            3 => Ping(Self::interpret_bytes::<mumble::Ping>(&payload)?),
            4 => Reject(Self::interpret_bytes::<mumble::Reject>(&payload)?),
            5 => ServerSync(Self::interpret_bytes::<mumble::ServerSync>(&payload)?),
            6 => ChannelRemove(Self::interpret_bytes::<mumble::ChannelRemove>(&payload)?),
            7 => ChannelState(Box::new(
                Self::interpret_bytes::<mumble::ChannelState>(&payload)?,
            )),
            8 => UserRemove(Self::interpret_bytes::<mumble::UserRemove>(&payload)?),
            9 => UserState(Box::new(
                Self::interpret_bytes::<mumble::UserState>(&payload)?,
            )),
            10 => BanList(Self::interpret_bytes::<mumble::BanList>(&payload)?),
            11 => TextMessage(Self::interpret_bytes::<mumble::TextMessage>(&payload)?),
            12 => PermissionDenied(
                Self::interpret_bytes::<mumble::PermissionDenied>(&payload)?,
            ),
            13 => ACL(Self::interpret_bytes::<mumble::ACL>(&payload)?),
            14 => QueryUsers(Self::interpret_bytes::<mumble::QueryUsers>(&payload)?),
            15 => CryptSetup(Self::interpret_bytes::<mumble::CryptSetup>(&payload)?),
            16 => ContextActionModify(
                Self::interpret_bytes::<mumble::ContextActionModify>(&payload)?,
            ),
            17 => {
                ContextAction(Self::interpret_bytes::<mumble::ContextAction>(&payload)?)
            }
            18 => UserList(Self::interpret_bytes::<mumble::UserList>(&payload)?),
            19 => VoiceTarget(Self::interpret_bytes::<mumble::VoiceTarget>(&payload)?),
            20 => PermissionQuery(
                Self::interpret_bytes::<mumble::PermissionQuery>(&payload)?,
            ),
            21 => CodecVersion(Self::interpret_bytes::<mumble::CodecVersion>(&payload)?),
            22 => UserStats(Box::new(
                Self::interpret_bytes::<mumble::UserStats>(&payload)?,
            )),
            23 => RequestBlob(Self::interpret_bytes::<mumble::RequestBlob>(&payload)?),
            24 => ServerConfig(Self::interpret_bytes::<mumble::ServerConfig>(&payload)?),
            25 => {
                SuggestConfig(Self::interpret_bytes::<mumble::SuggestConfig>(&payload)?)
            }
            _ => unreachable!(),
        })
    }

    /// Turns the message into bytes which can be sent.
    pub fn to_raw(&self) -> Vec<u8> {
        use crate::message_types::MessageType::*;
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
