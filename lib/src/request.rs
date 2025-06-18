use crate::criosphere::Criosphere;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    node_name: String,
    cluster_name: String,
    criosphere: Criosphere,
}

impl TryFrom<(Criosphere, String, String)> for Request {
    type Error = &'static str;

    fn try_from(value: (Criosphere, String, String)) -> Result<Self, Self::Error> {
        let (criosphere, node_name, cluster_name) = value;
        Ok(Request {
            node_name,
            cluster_name,
            criosphere,
        })
    }
}
