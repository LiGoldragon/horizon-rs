use crate::cluster::Cluster;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Horizon {
    cluster: Cluster,
    node: Node,
    ex_nodes: HashMap<String, Node>,
    users: Users,
}

impl TryFrom<(&Request)> for Horizon {
    type Error = &'static str;

    fn try_from(value: (&Data, String)) -> Result<Self, Self::Error> {
        Ok(Horizon {
            cluster: Cluster {},
        })
    }
}
