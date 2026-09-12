//! A cluster user seen from one node.

use super::names::{MagnitudeName, MagnitudeRanking, Named};
use crate::*;

/// The node a Horizon is projected for, as far as a user projection cares.
pub(crate) struct Viewpoint<'a> {
    /// The viewpoint node's name.
    pub node: &'a str,
    /// The cluster's first public domain, which addresses users.
    pub public_domain: &'a str,
    /// Whether the viewpoint node behaves as the cluster centre.
    pub center: bool,
    /// The viewpoint node's size, which caps every user's size.
    pub size: &'a str,
}

/// An authored user seen from one node of the cluster.
pub(crate) trait UserProjection {
    fn project_from(&self, trust: Magnitude, viewpoint: &Viewpoint<'_>) -> User;
}

impl UserProjection for UserDefinition {
    fn project_from(&self, trust: Magnitude, viewpoint: &Viewpoint<'_>) -> User {
        let viewpoint_key = self
            .user_pub_key_vector
            .iter()
            .find(|key| key.node_name == viewpoint.node);
        let is_code_dev = matches!(self.user_role, UserRole::Code | UserRole::Unlimited);
        let is_multimedia_dev =
            matches!(self.user_role, UserRole::Multimedia | UserRole::Unlimited);
        let is_max_trusted = matches!(trust, Magnitude::Max);
        let mut extra_groups = vec!["audio".to_owned()];
        if matches!(trust, Magnitude::Medium | Magnitude::Large | Magnitude::Max) {
            extra_groups.push("video".to_owned());
        }
        if is_max_trusted {
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
                .map(str::to_owned),
            );
        }
        let size = if self.magnitude.rank() <= viewpoint.size.magnitude_rank() {
            self.magnitude.name().to_owned()
        } else {
            viewpoint.size.to_owned()
        };
        User {
            name: self.user_name.clone(),
            role: self.user_role.name().into(),
            size,
            trust: trust.name().into(),
            keyboard: self.keyboard.name().into(),
            style: self.style.name().into(),
            github_id: Some(
                self.github_id_option
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| self.user_name.clone()),
            ),
            fast_repeat: self.boolean_option,
            public_keys: self
                .user_pub_key_vector
                .iter()
                .map(|key| UserPubKeyView {
                    node: key.node_name.clone(),
                    ssh: key.ssh_pub_key.clone(),
                    keygrip: key.keygrip.clone(),
                })
                .collect(),
            editor: self
                .editor_option
                .as_ref()
                .map(|editor| editor.name().to_owned()),
            text_size: self
                .text_size_option
                .as_ref()
                .map(|size| size.name().to_owned()),
            has_public_key: viewpoint_key.is_some(),
            email_address: format!("{}@{}", self.user_name, viewpoint.public_domain),
            matrix_id: format!("@{}:{}", self.user_name, viewpoint.public_domain),
            git_signing_key: viewpoint_key.map(|key| format!("&{}", key.keygrip)),
            use_colemak: matches!(self.keyboard, Keyboard::Colemak),
            use_fast_repeat: self.boolean_option.unwrap_or(true),
            is_multimedia_dev,
            is_code_dev,
            preferred_editor: self
                .editor_option
                .as_ref()
                .map(Named::name)
                .unwrap_or(if is_code_dev { "Emacs" } else { "Codium" })
                .to_owned(),
            resolved_text_size: self
                .text_size_option
                .as_ref()
                .map(Named::name)
                .unwrap_or("Medium")
                .to_owned(),
            ssh_public_keys: self
                .user_pub_key_vector
                .iter()
                .map(|key| format!("ssh-ed25519 {}", key.ssh_pub_key))
                .collect(),
            ssh_public_key: viewpoint_key.map(|key| format!("ssh-ed25519 {}", key.ssh_pub_key)),
            extra_groups,
            enable_linger: is_max_trusted && viewpoint.center,
        }
    }
}
