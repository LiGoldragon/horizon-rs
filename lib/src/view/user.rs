//! View-side `User` — per-user view from a viewpoint node, with every
//! computed field already filled.
//!
//! `UserProposal::project` (in `proposal::user`) is the constructor.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::magnitude::AtLeast;
use crate::name::{EmailAddress, GithubId, MatrixId, NodeName, UserName};
use crate::proposal::UserPubKeyEntry;
use crate::pub_key::SshPubKeyLine;
use crate::species::{Editor, Keyboard, Style, TextSize, UserSpecies};

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
    pub email_address: EmailAddress,
    pub matrix_id: MatrixId,
    /// `&<keygrip>` form, only when this user has a pubkey for the viewpoint node.
    pub git_signing_key: Option<String>,
    pub use_colemak: bool,
    pub use_fast_repeat: bool,
    pub is_multimedia_dev: bool,
    pub is_code_dev: bool,
    /// Resolved editor preference: the user's explicit `editor`
    /// when set, otherwise `Emacs` for code developers and `Codium`
    /// for everyone else.
    pub preferred_editor: Editor,
    /// User's preferred relative text size; consumers (ghostty,
    /// wezterm, emacs, codium) map this onto their own units.
    pub text_size: TextSize,
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
