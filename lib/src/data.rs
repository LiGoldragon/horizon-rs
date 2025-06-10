use crate::criosphere::Criosphere;
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

impl TryFrom<(&StructuredAttrs, String)> for Data {
    type Error = &'static str;

    fn try_from(value: (&StructuredAttrs, String)) -> Result<Self, Self::Error> {
        Ok(Data {
            species: String::new(),
        })
    }
}
