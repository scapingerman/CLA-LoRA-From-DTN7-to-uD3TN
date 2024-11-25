#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    #[prost(enumeration = "PacketType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "packet::Content", tags = "2, 3, 4, 5")]
    pub content: ::core::option::Option<packet::Content>,
}
/// Nested message and enum types in `Packet`.
pub mod packet {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "2")]
        Advertise(super::Advertise),
        #[prost(message, tag = "3")]
        BundleForward(super::BundleForward),
        #[prost(message, tag = "4")]
        PingPong(super::PingPong),
        #[prost(message, tag = "5")]
        SetConfig(super::SetConfig),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLngPos {
    #[prost(float, tag = "1")]
    pub lat: f32,
    #[prost(float, tag = "2")]
    pub lng: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XyPos {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericPos {
    #[prost(string, tag = "1")]
    pub generic: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoPos {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Advertise {
    #[prost(string, tag = "1")]
    pub node_name: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "6")]
    pub data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(oneof = "advertise::Position", tags = "2, 3, 4, 5")]
    pub position: ::core::option::Option<advertise::Position>,
}
/// Nested message and enum types in `Advertise`.
pub mod advertise {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Position {
        #[prost(message, tag = "2")]
        LatLng(super::LatLngPos),
        #[prost(message, tag = "3")]
        Xy(super::XyPos),
        #[prost(message, tag = "4")]
        Generic(super::GenericPos),
        #[prost(message, tag = "5")]
        NoPos(super::NoPos),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BundleForward {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub destination: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bundle_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub bundle_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "5")]
    pub data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingPong {
    #[prost(enumeration = "PingPongType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub node_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfig {
    #[prost(string, tag = "1")]
    pub destination: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigContent {
    #[prost(enumeration = "ConfigType", tag = "1")]
    pub r#type: i32,
    #[prost(bool, tag = "2")]
    pub persist: bool,
    #[prost(map = "string, string", tag = "3")]
    pub config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketType {
    TypeAdvertise = 0,
    TypeBundleForward = 1,
    TypePingPong = 2,
}
impl PacketType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PacketType::TypeAdvertise => "TYPE_ADVERTISE",
            PacketType::TypeBundleForward => "TYPE_BUNDLE_FORWARD",
            PacketType::TypePingPong => "TYPE_PING_PONG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_ADVERTISE" => Some(Self::TypeAdvertise),
            "TYPE_BUNDLE_FORWARD" => Some(Self::TypeBundleForward),
            "TYPE_PING_PONG" => Some(Self::TypePingPong),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PingPongType {
    TypePing = 0,
    TypePong = 1,
}
impl PingPongType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PingPongType::TypePing => "TYPE_PING",
            PingPongType::TypePong => "TYPE_PONG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_PING" => Some(Self::TypePing),
            "TYPE_PONG" => Some(Self::TypePong),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConfigType {
    TypeEcla = 0,
    TypeDtnd = 1,
    TypeLora = 2,
}
impl ConfigType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConfigType::TypeEcla => "TYPE_ECLA",
            ConfigType::TypeDtnd => "TYPE_DTND",
            ConfigType::TypeLora => "TYPE_LORA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_ECLA" => Some(Self::TypeEcla),
            "TYPE_DTND" => Some(Self::TypeDtnd),
            "TYPE_LORA" => Some(Self::TypeLora),
            _ => None,
        }
    }
}
