use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    name: String,
    size: String,
    species: String,
    machine: String,
    wireguard_pre_criome: String,
    node_ip: String,
    link_local_ips: String,
    trust: String,
    ssh: String,
    ygg_pre_criome: String,
    ygg_address: String,
    ygg_subnet: String,
    nix_pre_criome: String,
    criome_domain_name: String,
    system: String,
    nb_of_build_cores: String,
    type_is: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeMethods {
    is_fully_trusted: Trust,
    is_size_at_least: Size,
    is_builder: String,
    is_dispatcher: String,
    is_nix_cache: String,
    has_nix_precriad: String,
    has_ygg_precriad: String,
    has_ssh_precriad: String,
    has_wireguard_precriad: String,
    has_base_precriads: String,
    ssh_precriom: String,
    nix_pre_criome: String,
    nix_cache_domain: String,
    nix_url: String,
}
