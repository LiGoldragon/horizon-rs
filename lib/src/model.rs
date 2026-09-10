//! Datom-free, portable result of an actualized Horizon viewpoint.

use serde::Serialize;
use std::collections::BTreeMap;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Horizon {
    pub cluster: String,
    pub tailnet_base_domain: String,
    pub trusted_build_public_keys: Vec<String>,
    pub node: Node,
    pub ex_nodes: BTreeMap<String, Node>,
    pub users: Vec<User>,
    pub domains: Vec<Domain>,
    pub trust: Trust,
    pub domain_configuration: DomainConfigurationView,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub name: String,
    pub variant: String,
    pub is_live: bool,
    pub is_installation: bool,
    pub installation: Option<InstallationView>,
    pub installation_disks: Vec<Disk>,
    pub size: String,
    pub trust: String,
    pub online: Option<bool>,
    pub keyboard: String,
    pub compressed_swap_memory_percent: Option<i64>,
    pub machine: Machine,
    pub network: Network,
    pub keys: Keys,
    pub capabilities: Vec<Capability>,
    pub criome_domain_name: String,
    pub system: String,
    pub max_jobs: i64,
    pub build_cores: i64,
    pub is_fully_trusted: bool,
    pub is_remote_nix_builder: bool,
    pub is_dispatcher: bool,
    pub is_nix_cache: bool,
    pub is_large_edge: bool,
    pub enable_network_manager: bool,
    pub has_base_public_keys: bool,
    pub nix_public_key_line: Option<String>,
    pub nix_cache_domain: Option<String>,
    pub nix_url: Option<String>,
    pub behaves_as: BehavesAs,
    pub ssh_public_key_line: String,
    pub builder_configs: Vec<BuilderConfig>,
    pub cache_urls: Vec<String>,
    pub ex_nodes_ssh_public_keys: Vec<String>,
    pub dispatchers_ssh_public_keys: Vec<String>,
    pub admin_ssh_public_keys: Vec<String>,
    pub image_exchange_public_keys: Vec<String>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationView {
    pub bootloader: String,
    pub disks: Vec<Disk>,
    pub swap_devices: Vec<SwapDeviceView>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct Disk {
    pub device: String,
    pub mount: String,
    pub fs_type: String,
    pub options: Vec<String>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapDeviceView {
    pub device: String,
    pub size_mebibytes: Option<i64>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub kind: String,
    pub architecture: String,
    pub host: Option<String>,
    pub additional_hosts: Vec<String>,
    pub user: Option<String>,
    pub disk_gib: Option<i64>,
    pub hardware: HardwareView,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareView {
    pub cores: i64,
    pub model: Option<String>,
    pub motherboard: Option<String>,
    pub chip_generation: Option<i64>,
    pub ram_gib: Option<i64>,
    pub location: Option<String>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BehavesAs {
    pub center: bool,
    pub router: bool,
    pub edge: bool,
    pub next_generation: bool,
    pub low_power: bool,
    pub bare_metal: bool,
    pub virtual_machine: bool,
    pub iso: bool,
    pub large_ai: bool,
    pub test_vm: bool,
    pub cloud_node: bool,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderConfig {
    pub host_name: String,
    pub ssh_user: String,
    pub ssh_key: String,
    pub supported_features: Vec<String>,
    pub system: String,
    pub max_jobs: i64,
    pub public_host_key: String,
    pub public_host_key_line: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub link_local_ips: Vec<String>,
    pub node_ip: Option<String>,
    pub wireguard_public_key: Option<String>,
    pub wireguard_proxies: Vec<WireguardProxyView>,
    pub router_interfaces: Option<RouterInterfacesView>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WireguardProxyView {
    pub public_key: String,
    pub endpoint: String,
    pub interface_ip: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterInterfacesView {
    pub wan: String,
    pub wlan: String,
    pub wlan_band: String,
    pub wlan_channel: i64,
    pub wlan_standard: String,
    pub wpa3_sae_password_reference: Option<String>,
    pub backup_wireless: Option<BackupWirelessView>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWirelessView {
    pub interface: String,
    pub network_name: String,
    pub band: String,
    pub channel: i64,
    pub standard: String,
    pub password_reference: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub ssh: String,
    pub nix: Option<String>,
    pub yggdrasil: Option<YggdrasilKeyView>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YggdrasilKeyView {
    pub public_key: String,
    pub address: String,
    pub subnet: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Capability {
    Graphical,
    Center,
    LargeAi,
    Router,
    Edge,
    NextGeneration,
    LowPower,
    TestVm,
    VmTesting {
        gpu_passthrough: bool,
        display: String,
        gpu: Option<String>,
    },
    CloudNode,
    Printing,
    HardwareVideo,
    Nordvpn,
    WifiCertificate,
    TailnetClient,
    TailnetController,
    NixBuilder {
        maximum_jobs: Option<i64>,
    },
    NixCache,
    PersonaDevelopment {
        capabilities: Vec<String>,
    },
    VmHost {
        guest_subnet: String,
        kvm: String,
        maximum_guests: Option<i64>,
    },
    WebHost {
        sites: Vec<HostedSiteView>,
    },
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct HostedSiteView {
    pub domain: String,
    pub source: String,
    pub renderer: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub role: String,
    pub size: String,
    pub trust: String,
    pub keyboard: String,
    pub style: String,
    pub github_id: Option<String>,
    pub fast_repeat: Option<bool>,
    pub public_keys: Vec<UserPubKeyView>,
    pub editor: Option<String>,
    pub text_size: Option<String>,
    pub has_public_key: bool,
    pub email_address: String,
    pub matrix_id: String,
    pub git_signing_key: Option<String>,
    pub use_colemak: bool,
    pub use_fast_repeat: bool,
    pub is_multimedia_dev: bool,
    pub is_code_dev: bool,
    pub preferred_editor: String,
    pub resolved_text_size: String,
    pub ssh_public_keys: Vec<String>,
    pub ssh_public_key: Option<String>,
    pub extra_groups: Vec<String>,
    pub enable_linger: bool,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPubKeyView {
    pub node: String,
    pub ssh: String,
    pub keygrip: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct Domain {
    pub name: String,
    pub provider: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trust {
    pub cluster: String,
    pub clusters: Vec<TrustEntry>,
    pub nodes: Vec<TrustEntry>,
    pub users: Vec<TrustEntry>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct TrustEntry {
    pub name: String,
    pub magnitude: String,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainConfigurationView {
    pub internal_suffix: String,
    pub public_cluster_domains: Vec<String>,
}
