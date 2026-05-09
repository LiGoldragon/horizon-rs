//! Tests for `User` projection.
//!
//! Focused on the `preferred_editor` field's default-Emacs behaviour
//! (workspace's primary editor — unconditional default, not a smart
//! pick based on `is_code_dev`).

use std::collections::BTreeMap;

use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, NodeName, UserName};
use horizon_lib::proposal::UserProposal;
use horizon_lib::species::{Editor, Keyboard, Style, UserSpecies};
use horizon_lib::user::UserProjection;

fn fresh_proposal(species: UserSpecies, editor: Option<Editor>) -> UserProposal {
    UserProposal {
        species,
        size: Magnitude::Med,
        keyboard: Keyboard::Colemak,
        style: Style::Emacs,
        github_id: None,
        fast_repeat: None,
        pub_keys: BTreeMap::new(),
        editor,
    }
}

fn fresh_ctx<'a>(
    name: UserName,
    cluster: &'a ClusterName,
    node: &'a NodeName,
) -> UserProjection<'a> {
    UserProjection {
        name,
        cluster,
        viewpoint_node: node,
        trust: Magnitude::Max,
        viewpoint_behaves_as_center: true,
        viewpoint_node_size: Magnitude::Large,
    }
}

fn project(species: UserSpecies, editor: Option<Editor>) -> Editor {
    let proposal = fresh_proposal(species, editor);
    let cluster = ClusterName::try_new("goldragon").unwrap();
    let node = NodeName::try_new("ouranos").unwrap();
    let user_name = UserName::try_new("li").unwrap();
    proposal
        .project(fresh_ctx(user_name, &cluster, &node))
        .preferred_editor
}

#[test]
fn preferred_editor_defaults_to_emacs_when_proposal_omits_it() {
    assert_eq!(project(UserSpecies::Code, None), Editor::Emacs);
}

#[test]
fn preferred_editor_defaults_to_emacs_for_multimedia_users_too() {
    // Distinct from earlier "smart default": non-code-devs no longer
    // get Codium by default. Emacs is the workspace's primary editor
    // unconditionally; explicit `Codium` opt-in is required.
    assert_eq!(project(UserSpecies::Multimedia, None), Editor::Emacs);
}

#[test]
fn preferred_editor_passes_through_when_set_to_codium() {
    assert_eq!(project(UserSpecies::Code, Some(Editor::Codium)), Editor::Codium);
}

#[test]
fn preferred_editor_passes_through_when_set_to_emacs() {
    assert_eq!(project(UserSpecies::Code, Some(Editor::Emacs)), Editor::Emacs);
}
