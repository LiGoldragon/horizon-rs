//! Tests for `user::UserProposal::project` — derived booleans,
//! pubkey shadows, extra_groups, and editor smart defaults.

use std::collections::BTreeMap;

use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, GithubId, Keygrip, NodeName, PublicDomain, UserName};
use horizon_lib::proposal::{UserProjection, UserProposal, UserPubKeyEntry};
use horizon_lib::pub_key::SshPubKey;
use horizon_lib::species::{Editor, Keyboard, Style, TextSize, UserSpecies};

fn pubkey_entry() -> UserPubKeyEntry {
    UserPubKeyEntry {
        ssh: SshPubKey::try_new("AAAAC3NzaC1lZDI1NTE5AAAA").unwrap(),
        keygrip: Keygrip::try_new("0123456789ABCDEF0123456789ABCDEF01234567").unwrap(),
    }
}

fn user_proposal(species: UserSpecies, size: Magnitude, with_viewpoint_key: bool) -> UserProposal {
    let mut pub_keys = BTreeMap::new();
    if with_viewpoint_key {
        pub_keys.insert(NodeName::try_new("ouranos").unwrap(), pubkey_entry());
    }
    UserProposal {
        species,
        size,
        keyboard: Keyboard::Colemak,
        style: Style::Emacs,
        github_id: None,
        fast_repeat: None,
        pub_keys,
        editor: None,
        text_size: None,
    }
}

fn ctx<'cluster, 'name>(
    cluster: &'cluster ClusterName,
    viewpoint_node: &'name NodeName,
    trust: Magnitude,
    viewpoint_behaves_as_center: bool,
    viewpoint_node_size: Magnitude,
) -> UserProjection<'static>
where
    'cluster: 'static,
    'name: 'static,
{
    UserProjection {
        name: UserName::try_new("li").unwrap(),
        cluster,
        cluster_public_domain: public_domain(),
        viewpoint_node,
        trust,
        viewpoint_behaves_as_center,
        viewpoint_node_size,
    }
}

fn cluster() -> &'static ClusterName {
    static CLUSTER: std::sync::OnceLock<ClusterName> = std::sync::OnceLock::new();
    CLUSTER.get_or_init(|| ClusterName::try_new("goldragon").unwrap())
}

fn public_domain() -> &'static PublicDomain {
    static PUBLIC_DOMAIN: std::sync::OnceLock<PublicDomain> = std::sync::OnceLock::new();
    PUBLIC_DOMAIN.get_or_init(|| PublicDomain::try_new("criome.net").unwrap())
}

fn viewpoint() -> &'static NodeName {
    static NODE: std::sync::OnceLock<NodeName> = std::sync::OnceLock::new();
    NODE.get_or_init(|| NodeName::try_new("ouranos").unwrap())
}

#[test]
fn code_species_is_code_dev_and_smart_defaults_to_emacs() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(user.is_code_dev);
    assert!(matches!(user.preferred_editor, Editor::Emacs));
}

#[test]
fn multimedia_species_is_multimedia_dev_and_smart_defaults_to_codium() {
    let user = user_proposal(UserSpecies::Multimedia, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(user.is_multimedia_dev);
    assert!(!user.is_code_dev);
    assert!(matches!(user.preferred_editor, Editor::Codium));
}

#[test]
fn unlimited_species_is_both_code_and_multimedia_dev() {
    let user = user_proposal(UserSpecies::Unlimited, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(user.is_code_dev);
    assert!(user.is_multimedia_dev);
}

#[test]
fn explicit_editor_overrides_smart_default() {
    let mut proposal = user_proposal(UserSpecies::Code, Magnitude::Max, true);
    proposal.editor = Some(Editor::Codium);
    let user = proposal.project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(matches!(user.preferred_editor, Editor::Codium));
}

#[test]
fn user_size_floors_at_viewpoint_node_size() {
    // Max user on a Medium box → projected size is Medium.
    let user = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Medium,
    ));
    assert!(user.size.medium);
    assert!(!user.size.large);
}

#[test]
fn extra_groups_audio_only_at_low_trust() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Min, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Min,
        false,
        Magnitude::Max,
    ));
    assert_eq!(user.extra_groups, vec!["audio".to_string()]);
}

#[test]
fn extra_groups_add_video_at_medium_trust() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Min, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Medium,
        false,
        Magnitude::Max,
    ));
    assert!(user.extra_groups.contains(&"audio".to_string()));
    assert!(user.extra_groups.contains(&"video".to_string()));
    assert!(!user.extra_groups.contains(&"adbusers".to_string()));
}

#[test]
fn extra_groups_add_admin_set_at_max_trust() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Min, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        false,
        Magnitude::Max,
    ));
    for required in &[
        "audio",
        "video",
        "adbusers",
        "nixdev",
        "systemd-journal",
        "dialout",
        "plugdev",
        "power",
        "storage",
        "libvirtd",
    ] {
        assert!(
            user.extra_groups.contains(&required.to_string()),
            "missing required group: {required}",
        );
    }
}

#[test]
fn enable_linger_only_at_max_trust_on_center_viewpoint() {
    let on_center_max = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(on_center_max.enable_linger);

    let on_edge_max = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        false,
        Magnitude::Max,
    ));
    assert!(!on_edge_max.enable_linger);

    let on_center_med = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Medium,
        true,
        Magnitude::Max,
    ));
    assert!(!on_center_med.enable_linger);
}

#[test]
fn has_pub_key_true_when_viewpoint_node_in_pub_keys() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(user.has_pub_key);
    assert!(user.git_signing_key.as_ref().unwrap().starts_with('&'));
    assert!(user.ssh_pub_key.is_some());
}

#[test]
fn has_pub_key_false_when_viewpoint_node_absent_from_pub_keys() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Max, false).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(!user.has_pub_key);
    assert!(user.ssh_pub_key.is_none());
    assert!(user.git_signing_key.is_none());
}

#[test]
fn email_and_matrix_id_compose_from_name_and_cluster() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert_eq!(user.email_address.as_str(), "li@goldragon.criome.net");
    assert_eq!(user.matrix_id.as_str(), "@li:goldragon.criome.net");
}

#[test]
fn github_id_defaults_to_user_name_when_absent() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert_eq!(
        user.github_id.as_ref().unwrap(),
        &GithubId::try_new("li").unwrap(),
    );
}

#[test]
fn text_size_defaults_to_medium_when_absent() {
    let user = user_proposal(UserSpecies::Code, Magnitude::Max, true).project(ctx(
        cluster(),
        viewpoint(),
        Magnitude::Max,
        true,
        Magnitude::Max,
    ));
    assert!(matches!(user.text_size, TextSize::Medium));
}
