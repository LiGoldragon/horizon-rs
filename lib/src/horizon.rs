use crate::data::Data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Horizon {
    cluster: Cluster,
    node: Node,
    ex_nodes: HashMap<String, Node>,
    users: Users,
}

#[derive(Serialize, Deserialize)]
pub struct Cluster {
    name: String,
    methods: ClusterMethods,
}

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
    methods: NodeMethods,
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    name: String,
    style: String,
    species: String,
    keyboard: KeyboardLayout,
    size: Size,
    trust: Trust,
    pre_criomes: PreCriomes,
    github_id: GithubID,
    methods: Methods,
}

impl TryFrom<(&Data, String)> for Horizon {
    type Error = &'static str;

    fn try_from(value: (&Data, String)) -> Result<Self, Self::Error> {
        Ok(Horizon::new())
    }
}
