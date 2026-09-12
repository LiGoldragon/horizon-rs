#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type NodeName = String;
#[rustfmt::skip]
pub type ClusterName = String;
#[rustfmt::skip]
pub type UserName = String;
#[rustfmt::skip]
pub type DomainName = String;
#[rustfmt::skip]
pub type ModelName = String;
#[rustfmt::skip]
pub type Location = String;
#[rustfmt::skip]
pub type Latitude = datom_codec::Decimal;
#[rustfmt::skip]
pub type Longitude = datom_codec::Decimal;
#[rustfmt::skip]
pub type Altitude = datom_codec::Decimal;
#[rustfmt::skip]
pub type Accuracy = datom_codec::Decimal;
#[rustfmt::skip]
pub type Interface = String;
#[rustfmt::skip]
pub type WirelessNetworkName = String;
#[rustfmt::skip]
pub type SecretName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SecretReference {
    pub secret_name: SecretName,
}
#[rustfmt::skip]
pub type GithubId = String;
#[rustfmt::skip]
pub type Keygrip = String;
#[rustfmt::skip]
pub type DevicePath = String;
#[rustfmt::skip]
pub type MountPath = String;
#[rustfmt::skip]
pub type SshPubKey = String;
#[rustfmt::skip]
pub type NixPubKey = String;
#[rustfmt::skip]
pub type WireguardPubKey = String;
#[rustfmt::skip]
pub type YggPubKey = String;
#[rustfmt::skip]
pub type YggAddress = String;
#[rustfmt::skip]
pub type YggSubnet = String;
#[rustfmt::skip]
pub type LinkLocalIp = String;
#[rustfmt::skip]
pub type NodeIp = String;
#[rustfmt::skip]
pub type TapSubnet = String;
#[rustfmt::skip]
pub type SiteSource = String;
#[rustfmt::skip]
pub type ServedDomain = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UserRole {
    Code,
    Multimedia,
    Unlimited,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Architecture {
    X86_64,
    Arm64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Magnitude {
    Zero,
    Min,
    Medium,
    Large,
    Max,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Keyboard {
    Qwerty,
    Colemak,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Style {
    Vim,
    Emacs,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Editor {
    Codium,
    Emacs,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TextSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Bootloader {
    Uefi,
    Mbr,
    Uboot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MotherBoard {
    Ondyfaind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DomainProvider {
    Cloudflare,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum WlanBand {
    TwoG,
    FiveG,
    SixG,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum WlanStandard {
    Wifi4,
    Wifi6,
    Wifi7,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum KvmAvailability {
    Available,
    Absent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SiteRenderer {
    MarkdownStatic,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FsType {
    Ext2,
    Ext3,
    Ext4,
    Btrfs,
    Xfs,
    Zfs,
    F2fs,
    Bcachefs,
    Vfat,
    Exfat,
    Ntfs,
    Tmpfs,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Hardware {
    pub integer: i64,
    pub model_name_option: Option<ModelName>,
    pub mother_board_option: Option<MotherBoard>,
    pub first_integer_option: Option<i64>,
    pub second_integer_option: Option<i64>,
    pub location_option: Option<Location>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Cluster_Data {
    pub node_name: NodeName,
    pub node_name_vector: std::vec::Vec<NodeName>,
    pub user_name_option: Option<UserName>,
    pub architecture_option: Option<Architecture>,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct External_Data {
    pub string: String,
    pub architecture: Architecture,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VirtualMachineHost {
    Cluster(Cluster_Data),
    External(External_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Metal_Data {
    pub architecture: Architecture,
    pub hardware: Hardware,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VirtualMachine_Data {
    pub virtual_machine_host: VirtualMachineHost,
    pub hardware: Hardware,
    pub integer_option: Option<i64>,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MachineDefinition {
    Metal(Metal_Data),
    VirtualMachine(VirtualMachine_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DiskLayout {
    pub device_path: DevicePath,
    pub mount_path: MountPath,
    pub fs_type: FsType,
    pub string_vector: std::vec::Vec<String>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SwapDevice {
    pub device_path: DevicePath,
    pub integer_option: Option<i64>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompressedSwap {
    pub integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FixedLocation {
    pub first_decimal: datom_codec::Decimal,
    pub second_decimal: datom_codec::Decimal,
    pub third_decimal: datom_codec::Decimal,
    pub fourth_decimal: datom_codec::Decimal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NodeEnvironment {
    pub keyboard: Keyboard,
    pub compressed_swap_option: Option<CompressedSwap>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Installation {
    pub bootloader: Bootloader,
    pub disk_layout_vector: std::vec::Vec<DiskLayout>,
    pub swap_device_vector: std::vec::Vec<SwapDevice>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LiveDefinition {}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NodeVariant {
    Live(LiveDefinition),
    Installation(Installation),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct YggdrasilKey {
    pub ygg_pub_key: YggPubKey,
    pub ygg_address: YggAddress,
    pub ygg_subnet: YggSubnet,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NodeKeys {
    pub ssh_pub_key: SshPubKey,
    pub nix_pub_key_option: Option<NixPubKey>,
    pub yggdrasil_key_option: Option<YggdrasilKey>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WireguardProxy {
    pub wireguard_pub_key: WireguardPubKey,
    pub string: String,
    pub node_ip: NodeIp,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BackupWireless {
    pub interface: Interface,
    pub wireless_network_name: WirelessNetworkName,
    pub wlan_band: WlanBand,
    pub integer: i64,
    pub wlan_standard: WlanStandard,
    pub secret_reference: SecretReference,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RouterInterfaces {
    pub first_interface: Interface,
    pub second_interface: Interface,
    pub wlan_band: WlanBand,
    pub integer: i64,
    pub wlan_standard: WlanStandard,
    pub secret_reference_option: Option<SecretReference>,
    pub backup_wireless_option: Option<BackupWireless>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NodeNetwork {
    pub link_local_ip_vector: std::vec::Vec<LinkLocalIp>,
    pub node_ip_option: Option<NodeIp>,
    pub wireguard_pub_key_option: Option<WireguardPubKey>,
    pub wireguard_proxy_vector: std::vec::Vec<WireguardProxy>,
    pub router_interfaces_option: Option<RouterInterfaces>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HostedSite {
    pub served_domain: ServedDomain,
    pub site_source: SiteSource,
    pub site_renderer: SiteRenderer,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PersonaCapability {
    GitoliteServer,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NoSettings {}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VmTesting_Data {
    pub boolean: bool,
    pub string: String,
    pub string_option: Option<String>,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VmHost_Data {
    pub tap_subnet: TapSubnet,
    pub kvm_availability: KvmAvailability,
    pub integer_option: Option<i64>,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NodeCapability {
    Graphical(NoSettings),
    Center(NoSettings),
    LargeAi(NoSettings),
    Router(NoSettings),
    Edge(NoSettings),
    NextGeneration(NoSettings),
    LowPower(NoSettings),
    TestVm(NoSettings),
    VmTesting(VmTesting_Data),
    CloudNode(NoSettings),
    Printing(NoSettings),
    HardwareVideo(NoSettings),
    Nordvpn(NoSettings),
    WifiCertificate(NoSettings),
    TailnetClient(NoSettings),
    TailnetController(NoSettings),
    NixBuilder(Option<i64>),
    NixCache(NoSettings),
    PersonaDevelopment(std::vec::Vec<PersonaCapability>),
    VmHost(VmHost_Data),
    WebHost(std::vec::Vec<HostedSite>),
}
#[rustfmt::skip]
pub type Capabilities = std::vec::Vec<NodeCapability>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NodeDefinition {
    pub node_name: NodeName,
    pub node_variant: NodeVariant,
    pub first_magnitude: Magnitude,
    pub second_magnitude: Magnitude,
    pub machine_definition: MachineDefinition,
    pub node_environment: NodeEnvironment,
    pub node_network: NodeNetwork,
    pub node_keys: NodeKeys,
    pub boolean_option: Option<bool>,
    pub capabilities: Capabilities,
    pub fixed_location_option: Option<FixedLocation>,
}
#[rustfmt::skip]
pub type GenericNodeNames = std::vec::Vec<NodeName>;
#[rustfmt::skip]
pub type GenericNodes = std::vec::Vec<NodeDefinition>;
#[rustfmt::skip]
pub type ClusterNodes = std::vec::Vec<NodeDefinition>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UserPubKey {
    pub node_name: NodeName,
    pub ssh_pub_key: SshPubKey,
    pub keygrip: Keygrip,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UserDefinition {
    pub user_name: UserName,
    pub user_role: UserRole,
    pub magnitude: Magnitude,
    pub keyboard: Keyboard,
    pub style: Style,
    pub github_id_option: Option<GithubId>,
    pub boolean_option: Option<bool>,
    pub user_pub_key_vector: std::vec::Vec<UserPubKey>,
    pub editor_option: Option<Editor>,
    pub text_size_option: Option<TextSize>,
}
#[rustfmt::skip]
pub type Users = std::vec::Vec<UserDefinition>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DomainDefinition {
    pub domain_name: DomainName,
    pub domain_provider: DomainProvider,
}
#[rustfmt::skip]
pub type Domains = std::vec::Vec<DomainDefinition>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClusterTrustEntry {
    pub cluster_name: ClusterName,
    pub magnitude: Magnitude,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NodeTrustEntry {
    pub node_name: NodeName,
    pub magnitude: Magnitude,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UserTrustEntry {
    pub user_name: UserName,
    pub magnitude: Magnitude,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClusterTrust {
    pub magnitude: Magnitude,
    pub cluster_trust_entry_vector: std::vec::Vec<ClusterTrustEntry>,
    pub node_trust_entry_vector: std::vec::Vec<NodeTrustEntry>,
    pub user_trust_entry_vector: std::vec::Vec<UserTrustEntry>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DomainConfiguration {
    pub string: String,
    pub domain_name_vector: std::vec::Vec<DomainName>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClusterDefinition {
    pub cluster_name: ClusterName,
    pub cluster_nodes: ClusterNodes,
    pub generic_node_names: GenericNodeNames,
    pub users: Users,
    pub domains: Domains,
    pub cluster_trust: ClusterTrust,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HorizonConfiguration {
    pub generic_nodes: GenericNodes,
    pub domain_configuration: DomainConfiguration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HorizonDefinition {
    pub horizon_configuration: HorizonConfiguration,
    pub cluster_definition: ClusterDefinition,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompositionRequest {
    pub first_string: String,
    pub second_string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CompositionCommand {
    Compose(CompositionRequest),
}
