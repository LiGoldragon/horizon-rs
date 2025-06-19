use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    name: String,
    methods: ClusterMethods,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterMethods {
    name: String,
    trusted_build_pre_criomes: TrustedBuildPreCriomes,
}

#[derive(Default, Serialize, Deserialize)]
struct TrustedBuildPreCriomes(Vec<String>);
