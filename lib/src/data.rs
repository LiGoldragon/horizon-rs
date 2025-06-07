use crate::nix::StructuredAttrs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    node_name: String,
    cluster_name: String,
    criosphere: Criosphere,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Criosphere(HashMap<String, Cluster>);

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

impl TryFrom<(&StructuredAttrs, String)> for Data {
    type Error = &'static str;

    fn try_from(value: (&StructuredAttrs, String)) -> Result<Self, Self::Error> {
        Ok(Data {
            species: String::new(),
        })
    }
}
