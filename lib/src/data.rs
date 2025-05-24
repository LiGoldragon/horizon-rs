use crate::nix::StructuredAttrs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Data {}

impl TryFrom<(&StructuredAttrs, String)> for Data {
    type Error = &'static str;

    fn try_from(value: (&StructuredAttrs, String)) -> Result<Self, Self::Error> {
        Ok(Data::new())
    }
}
