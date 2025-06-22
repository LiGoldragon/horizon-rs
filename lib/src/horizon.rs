use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::cluster::{Cluster, ClusterMethods};
pub use crate::node::{Node, NodeMethods};
pub use crate::user::{User, UserMethods};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Horizon {
    cluster: Cluster,
    node: Node,
    ex_nodes: HashMap<String, Node>,
    users: Users,
}

#[derive(Default, Serialize, Deserialize)]
struct Users(HashMap<String, User>);

impl TryFrom<(Request)> for Horizon {
    type Error = &'static str;

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(Horizon {
            cluster: Cluster {},
        })
    }
}
