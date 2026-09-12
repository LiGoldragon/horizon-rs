//! Authored values whose view follows from the value alone.

use super::names::Named;
use crate::*;

/// One authored value's projection into the portable view it determines by
/// itself — no cluster context, no way to refuse.
pub(crate) trait Projection {
    type View;

    fn project(&self) -> Self::View;
}

impl Projection for DiskLayout {
    type View = Disk;

    fn project(&self) -> Disk {
        Disk {
            device: self.device_path.clone(),
            mount: self.mount_path.clone(),
            fs_type: self.fs_type.name().to_owned(),
            options: self.string_vector.clone(),
        }
    }
}

impl Projection for Hardware {
    type View = HardwareView;

    fn project(&self) -> HardwareView {
        HardwareView {
            cores: self.integer,
            model: self.model_name_option.clone(),
            motherboard: self
                .mother_board_option
                .as_ref()
                .map(|board| board.name().to_owned()),
            chip_generation: self.first_integer_option,
            ram_gib: self.second_integer_option,
            location: self.location_option.clone(),
        }
    }
}

impl Projection for NodeNetwork {
    type View = Network;

    fn project(&self) -> Network {
        Network {
            link_local_ips: self.link_local_ip_vector.clone(),
            node_ip: self.node_ip_option.clone(),
            wireguard_public_key: self.wireguard_pub_key_option.clone(),
            wireguard_proxies: self
                .wireguard_proxy_vector
                .iter()
                .map(|proxy| WireguardProxyView {
                    public_key: proxy.wireguard_pub_key.clone(),
                    endpoint: proxy.string.clone(),
                    interface_ip: proxy.node_ip.clone(),
                })
                .collect(),
            router_interfaces: self
                .router_interfaces_option
                .as_ref()
                .map(Projection::project),
        }
    }
}

impl Projection for RouterInterfaces {
    type View = RouterInterfacesView;

    fn project(&self) -> RouterInterfacesView {
        RouterInterfacesView {
            wan: self.first_interface.clone(),
            wlan: self.second_interface.clone(),
            wlan_band: self.wlan_band.name().into(),
            wlan_channel: self.integer,
            wlan_standard: self.wlan_standard.name().into(),
            wpa3_sae_password_reference: self
                .secret_reference_option
                .as_ref()
                .map(|reference| reference.secret_name.clone()),
            backup_wireless: self.backup_wireless_option.as_ref().map(|backup| {
                BackupWirelessView {
                    interface: backup.interface.clone(),
                    network_name: backup.wireless_network_name.clone(),
                    band: backup.wlan_band.name().into(),
                    channel: backup.integer,
                    standard: backup.wlan_standard.name().into(),
                    password_reference: backup.secret_reference.secret_name.clone(),
                }
            }),
        }
    }
}

impl Projection for NodeKeys {
    type View = Keys;

    fn project(&self) -> Keys {
        Keys {
            ssh: self.ssh_pub_key.clone(),
            nix: self.nix_pub_key_option.clone(),
            yggdrasil: self
                .yggdrasil_key_option
                .as_ref()
                .map(|key| YggdrasilKeyView {
                    public_key: key.ygg_pub_key.clone(),
                    address: key.ygg_address.clone(),
                    subnet: key.ygg_subnet.clone(),
                }),
        }
    }
}

impl Projection for NodeCapability {
    type View = Capability;

    fn project(&self) -> Capability {
        match self {
            NodeCapability::Graphical(_) => Capability::Graphical,
            NodeCapability::Center(_) => Capability::Center,
            NodeCapability::LargeAi(_) => Capability::LargeAi,
            NodeCapability::Router(_) => Capability::Router,
            NodeCapability::Edge(_) => Capability::Edge,
            NodeCapability::NextGeneration(_) => Capability::NextGeneration,
            NodeCapability::LowPower(_) => Capability::LowPower,
            NodeCapability::TestVm(_) => Capability::TestVm,
            NodeCapability::VmTesting(data) => Capability::VmTesting {
                gpu_passthrough: data.boolean,
                display: data.string.clone(),
                gpu: data.string_option.clone(),
            },
            NodeCapability::CloudNode(_) => Capability::CloudNode,
            NodeCapability::Printing(_) => Capability::Printing,
            NodeCapability::HardwareVideo(_) => Capability::HardwareVideo,
            NodeCapability::Nordvpn(_) => Capability::Nordvpn,
            NodeCapability::WifiCertificate(_) => Capability::WifiCertificate,
            NodeCapability::TailnetClient(_) => Capability::TailnetClient,
            NodeCapability::TailnetController(_) => Capability::TailnetController,
            NodeCapability::NixBuilder(maximum_jobs) => Capability::NixBuilder {
                maximum_jobs: *maximum_jobs,
            },
            NodeCapability::NixCache(_) => Capability::NixCache,
            NodeCapability::PersonaDevelopment(values) => Capability::PersonaDevelopment {
                capabilities: values.iter().map(|value| value.name().to_owned()).collect(),
            },
            NodeCapability::VmHost(data) => Capability::VmHost {
                guest_subnet: data.tap_subnet.clone(),
                kvm: data.kvm_availability.name().into(),
                maximum_guests: data.integer_option,
            },
            NodeCapability::WebHost(sites) => Capability::WebHost {
                sites: sites
                    .iter()
                    .map(|site| HostedSiteView {
                        domain: site.served_domain.clone(),
                        source: site.site_source.clone(),
                        renderer: site.site_renderer.name().into(),
                    })
                    .collect(),
            },
        }
    }
}

impl Projection for DomainDefinition {
    type View = Domain;

    fn project(&self) -> Domain {
        Domain {
            name: self.domain_name.clone(),
            provider: self.domain_provider.name().into(),
        }
    }
}

impl Projection for ClusterTrust {
    type View = Trust;

    fn project(&self) -> Trust {
        Trust {
            cluster: self.magnitude.name().into(),
            clusters: self
                .cluster_trust_entry_vector
                .iter()
                .map(|entry| TrustEntry {
                    name: entry.cluster_name.clone(),
                    magnitude: entry.magnitude.name().into(),
                })
                .collect(),
            nodes: self
                .node_trust_entry_vector
                .iter()
                .map(|entry| TrustEntry {
                    name: entry.node_name.clone(),
                    magnitude: entry.magnitude.name().into(),
                })
                .collect(),
            users: self
                .user_trust_entry_vector
                .iter()
                .map(|entry| TrustEntry {
                    name: entry.user_name.clone(),
                    magnitude: entry.magnitude.name().into(),
                })
                .collect(),
        }
    }
}
