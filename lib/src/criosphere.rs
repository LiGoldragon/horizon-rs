use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::horizon::{Cluster, Node, User};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Criosphere {
    version: CrioSphereVersion,
    clusters: HashMap<String, Cluster>,
}

#[derive(Serialize, Deserialize)]
pub enum CrioSphereVersion {
    #[serde(rename = "0")]
    Zero,
    #[serde(rename = "1")]
    One,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    species: Species,
    machine: Machine,
    pre_criomes: PreCriomes,
    node_ip: String,
    link_local_ips: String,
    trust: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Species {
    Center,
    Hybrid,
    Edge,
    EdgeTesting,
    MediaBroadcast,
    Router,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreCriomes {
    ssh: String,
    yggdrasil: String,
    nix: String,
}
