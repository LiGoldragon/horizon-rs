//! Output `User`: per-user view from a viewpoint node, with every
//! computed field already filled.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::magnitude::{AtLeast, Magnitude};
use crate::name::{ClusterName, GithubId, NodeName, UserName};
use crate::proposal::{UserProposal, UserPubKeyEntry};
use crate::pub_key::SshPubKeyLine;
use crate::species::{Keyboard, Style, UserSpecies};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    // input pass-through
    pub name: UserName,
    pub species: UserSpecies,
    pub size: AtLeast,
    pub trust: AtLeast,
    pub keyboard: Keyboard,
    pub style: Style,
    pub github_id: Option<GithubId>,
    pub pub_keys: BTreeMap<NodeName, UserPubKeyEntry>,

    // derived
    pub has_pub_key: bool,
    pub email_address: String,
    pub matrix_id: String,
    /// `&<keygrip>` form, only when this user has a pubkey for the viewpoint node.
    pub git_signing_key: Option<String>,
    pub use_colemak: bool,
    pub use_fast_repeat: bool,
    pub is_multimedia_dev: bool,
    pub is_code_dev: bool,
    pub ssh_pub_keys: Vec<SshPubKeyLine>,
    /// Viewpoint-node line, only when has_pub_key.
    pub ssh_pub_key: Option<SshPubKeyLine>,

    // derived node-contextual fields (depend on the viewpoint node role
    // as well as the user's trust level)
    /// Secondary Unix groups this user should be added to on the
    /// viewpoint node, derived from trust. Nix consumers still add
    /// dynamic groups (e.g. "sway" when `config.programs.sway.enable`)
    /// on top of this list.
    pub extra_groups: Vec<String>,
    /// `users.users.<u>.linger` — keep this user's systemd --user
    /// sessions alive. True iff `trust == Max` and the viewpoint node
    /// behaves as a `center`.
    pub enable_linger: bool,
}

pub struct UserProjection<'a> {
    pub name: UserName,
    pub cluster: &'a ClusterName,
    pub viewpoint_node: &'a NodeName,
    pub trust: Magnitude,
    /// Whether the projection's viewpoint node behaves as a `center`.
    /// Needed to derive `enable_linger`.
    pub viewpoint_behaves_as_center: bool,
    /// Capacity ceiling: the user's projected `size` is the floor of
    /// the user's declared size and the viewpoint node's declared
    /// size. A Max user on a Large box gets a Large-shaped home.
    /// Mirrors archive behavior (mkHorizonModule.nix `lowestOf [
    /// inputUser.size node.size ]`) which was lost in the Rust port.
    pub viewpoint_node_size: Magnitude,
}

impl UserProposal {
    pub fn project(&self, ctx: UserProjection<'_>) -> User {
        let github_id = self.github_id.clone().unwrap_or_else(|| {
            // Default to the user's own name when github_id is absent.
            GithubId::try_new(ctx.name.as_str()).expect("UserName is non-empty")
        });

        let viewpoint_entry = self.pub_keys.get(ctx.viewpoint_node);
        let has_pub_key = viewpoint_entry.is_some();
        let git_signing_key = viewpoint_entry.map(|e| format!("&{}", e.keygrip));
        let ssh_pub_key = viewpoint_entry.map(|e| e.ssh.line());

        let ssh_pub_keys: Vec<SshPubKeyLine> =
            self.pub_keys.values().map(|e| e.ssh.line()).collect();

        let email_address = format!("{}@{}.criome.net", ctx.name, ctx.cluster);
        let matrix_id = format!("@{}:{}.criome.net", ctx.name, ctx.cluster);

        let trust_ladder = ctx.trust.ladder();
        let mut extra_groups: Vec<String> = vec!["audio".into()];
        if trust_ladder.at_least_med {
            extra_groups.push("video".into());
        }
        if trust_ladder.at_least_max {
            extra_groups.extend(
                [
                    "adbusers",
                    "nixdev",
                    "systemd-journal",
                    "dialout",
                    "plugdev",
                    "power",
                    "storage",
                    "libvirtd",
                ]
                .into_iter()
                .map(String::from),
            );
        }
        let enable_linger = trust_ladder.at_least_max && ctx.viewpoint_behaves_as_center;

        User {
            has_pub_key,
            email_address,
            matrix_id,
            git_signing_key,
            use_colemak: matches!(self.keyboard, Keyboard::Colemak),
            use_fast_repeat: self.fast_repeat.unwrap_or(true),
            is_multimedia_dev: matches!(self.species, UserSpecies::Multimedia | UserSpecies::Unlimited),
            is_code_dev: matches!(self.species, UserSpecies::Code | UserSpecies::Unlimited),
            ssh_pub_keys,
            ssh_pub_key,
            extra_groups,
            enable_linger,

            name: ctx.name,
            species: self.species,
            size: self.size.floor(ctx.viewpoint_node_size).ladder(),
            trust: trust_ladder,
            keyboard: self.keyboard,
            style: self.style,
            github_id: Some(github_id),
            pub_keys: self.pub_keys.clone(),
        }
    }
}
