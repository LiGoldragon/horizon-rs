//! Node placement — where a node physically (Metal) or logically
//! (Contained inside another node) lives.

use nota_codec::NotaSum;
use serde::{Deserialize, Serialize};

use crate::name::{NodeName, UserName};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum NodePlacement {
    Metal {},
    Contained { host: NodeName, user: UserName },
}
