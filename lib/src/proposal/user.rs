//! Proposal-side `UserProposal` — the per-user input shape goldragon
//! emits.
//!
//! `UserProposal::project` is the constructor for `view::User`.

use std::collections::BTreeMap;

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::magnitude::Magnitude;
use crate::name::{ClusterName, GithubId, Keygrip, NodeName, UserName};
use crate::pub_key::{SshPubKey, SshPubKeyLine};
use crate::species::{Editor, Keyboard, Style, TextSize, UserSpecies};
use crate::view;

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct UserProposal {
    pub species: UserSpecies,
    #[serde(default = "Magnitude::default_zero")]
    pub size: Magnitude,
    pub keyboard: Keyboard,
    pub style: Style,
    #[serde(default)]
    pub github_id: Option<GithubId>,
    /// `None` means default-true; preserved to distinguish absent from explicit-true.
    #[serde(default)]
    pub fast_repeat: Option<bool>,
    #[serde(default)]
    pub pub_keys: BTreeMap<NodeName, UserPubKeyEntry>,
    /// Preferred top-level editor application. `None` means use the
    /// projection's smart default (`Emacs` for code developers,
    /// `Codium` otherwise).
    #[serde(default)]
    pub editor: Option<Editor>,
    /// Preferred relative text size — drives terminal font, editor
    /// font, and editor UI zoom. `None` means use the default
    /// (`Medium`).
    #[serde(default)]
    pub text_size: Option<TextSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct UserPubKeyEntry {
    pub ssh: SshPubKey,
    pub keygrip: Keygrip,
}

pub struct UserProjection<'a> {
    pub name: UserName,
    pub cluster: &'a ClusterName,
    pub cluster_public_domain: &'a str,
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
    pub fn project(&self, ctx: UserProjection<'_>) -> view::User {
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

        let email_address = format!("{}@{}.{}", ctx.name, ctx.cluster, ctx.cluster_public_domain);
        let matrix_id = format!("@{}:{}.{}", ctx.name, ctx.cluster, ctx.cluster_public_domain);

        let trust_ladder = ctx.trust.ladder();
        let mut extra_groups: Vec<String> = vec!["audio".into()];
        if trust_ladder.medium {
            extra_groups.push("video".into());
        }
        if trust_ladder.max {
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
        let enable_linger = trust_ladder.max && ctx.viewpoint_behaves_as_center;

        let is_code_dev = matches!(self.species, UserSpecies::Code | UserSpecies::Unlimited);
        let preferred_editor = self.editor.unwrap_or(if is_code_dev {
            Editor::Emacs
        } else {
            Editor::Codium
        });

        view::User {
            has_pub_key,
            email_address,
            matrix_id,
            git_signing_key,
            use_colemak: matches!(self.keyboard, Keyboard::Colemak),
            use_fast_repeat: self.fast_repeat.unwrap_or(true),
            is_multimedia_dev: matches!(self.species, UserSpecies::Multimedia | UserSpecies::Unlimited),
            is_code_dev,
            preferred_editor,
            text_size: self.text_size.unwrap_or_default(),
            ssh_pub_keys,
            ssh_pub_key,
            extra_groups,
            enable_linger,

            name: ctx.name,
            species: self.species,
            size: self.size.min(ctx.viewpoint_node_size).ladder(),
            trust: trust_ladder,
            keyboard: self.keyboard,
            style: self.style,
            github_id: Some(github_id),
            pub_keys: self.pub_keys.clone(),
        }
    }
}
