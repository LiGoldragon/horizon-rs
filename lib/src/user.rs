use serde::{Deserialize, Serialize};

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
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMethods {
    pre_criomes: PreCriomes,
}
