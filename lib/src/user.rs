use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct Users(Hashmap<String, User>);

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
    methods: Methods,
}
