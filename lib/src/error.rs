use thiserror::Error;

use crate::name::NodeName;
use crate::proposal::secret::SecretName;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid name: {kind} cannot be empty")]
    EmptyName { kind: &'static str },

    #[error("invalid secret name {got:?} — must be non-empty ASCII letters, digits, or dashes")]
    InvalidSecretName { got: String },

    #[error(
        "invalid AI provider name {got:?} — must be non-empty ASCII letters, digits, or dashes"
    )]
    InvalidAiProviderName { got: String },

    #[error(
        "invalid AI model id {got:?} — must be non-empty ASCII letters, digits, dashes, dots, or underscores"
    )]
    InvalidAiModelId { got: String },

    #[error(
        "invalid NordVPN server name {got:?} — must be non-empty ASCII letters, digits, or dashes"
    )]
    InvalidNordvpnServerName { got: String },

    #[error(
        "invalid VPN country code {got:?} — must be ISO 3166-1 alpha-2 (two ASCII uppercase letters)"
    )]
    InvalidVpnCountryCode { got: String },

    #[error(
        "invalid ISO 3166-1 alpha-2 country code {got:?} — must be exactly two ASCII uppercase letters"
    )]
    InvalidIsoCountryCode { got: String },

    #[error("invalid Wi-Fi SSID {got:?} — must be 1 to 32 bytes (IEEE 802.11 limit)")]
    InvalidSsid { got: String },

    #[error("invalid email address {got:?} — must contain `@`")]
    InvalidEmailAddress { got: String },

    #[error("invalid Matrix ID {got:?} — must start with `@` and contain `:`")]
    InvalidMatrixId { got: String },

    #[error("invalid keygrip: expected 40 hex chars, got {got:?}")]
    InvalidKeygrip { got: String },

    #[error("invalid hex public key (expected {expected_len} hex chars): {got:?}")]
    InvalidHexKey { expected_len: usize, got: String },

    #[error("invalid base64 public key (expected {expected_len} chars): {got:?}")]
    InvalidBase64Key { expected_len: usize, got: String },

    #[error("invalid yggdrasil address {got:?}: {source}")]
    InvalidYggAddress {
        got: String,
        source: std::net::AddrParseError,
    },

    #[error("yggdrasil subnet must not be empty")]
    EmptyYggSubnet,

    #[error("invalid node ip {got:?}: {source}")]
    InvalidNodeIp {
        got: String,
        source: ipnet::AddrParseError,
    },

    #[error("invalid ip address {got:?}: {source}")]
    InvalidIpAddress {
        got: String,
        source: std::net::AddrParseError,
    },

    #[error("invalid lan cidr {got:?}: {source}")]
    InvalidLanCidr {
        got: String,
        source: ipnet::AddrParseError,
    },

    #[error("unknown {kind}: {got:?}")]
    UnknownVariant { kind: &'static str, got: String },

    #[error("cluster has no node {0:?}")]
    NodeNotInCluster(NodeName),

    #[error("multiple tailnet controller servers: {first:?} and {second:?}")]
    MultipleTailnetControllers { first: NodeName, second: NodeName },

    #[error(
        "tailnet controller declared on node {node:?} but cluster.tailnet is unset (base_domain required)"
    )]
    TailnetControllerWithoutClusterConfig { node: NodeName },

    #[error(
        "duplicate cluster secret binding for {name:?} — every SecretName must be bound exactly once"
    )]
    DuplicateSecretBinding { name: SecretName },

    #[error("invalid public certificate {got:?} — must start with -----BEGIN CERTIFICATE-----")]
    InvalidPublicCertificate { got: String },

    #[error("pod node {0:?} references missing super-node {1:?}")]
    MissingSuperNode(NodeName, NodeName),

    #[error("pod node {0:?} has no super-node and no arch of its own")]
    UnresolvableArch(NodeName),

    #[error("nota: {0}")]
    Nota(#[from] nota_codec::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
