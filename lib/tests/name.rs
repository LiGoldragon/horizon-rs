//! Tests for `name` — typed name newtypes and `ModelName::known()`
//! typed dispatch.

use horizon_lib::name::{
    ClusterDomain, ClusterName, CriomeDomainName, DomainName, GithubId, Keygrip, ModelName,
    NodeName, UserName,
};
use horizon_lib::species::KnownModel;

#[test]
fn cluster_name_accepts_non_empty() {
    let name = ClusterName::try_new("goldragon").unwrap();
    assert_eq!(name.as_str(), "goldragon");
}

#[test]
fn cluster_name_rejects_empty() {
    let error = ClusterName::try_new("").unwrap_err();
    assert!(error.to_string().contains("cluster name"));
}

#[test]
fn node_name_accepts_non_empty() {
    let name = NodeName::try_new("ouranos").unwrap();
    assert_eq!(name.as_str(), "ouranos");
    assert_eq!(format!("{name}"), "ouranos");
}

#[test]
fn node_name_rejects_empty() {
    assert!(NodeName::try_new("").is_err());
}

#[test]
fn node_name_implements_from_str() {
    let name: NodeName = "tiger".parse().unwrap();
    assert_eq!(name.as_str(), "tiger");
}

#[test]
fn user_name_round_trip() {
    let name = UserName::try_new("li").unwrap();
    assert_eq!(name.as_str(), "li");
}

#[test]
fn model_name_known_returns_typed_known_model() {
    assert_eq!(
        ModelName::try_new("ThinkPadX230").unwrap().known(),
        Some(KnownModel::ThinkPadX230),
    );
    assert_eq!(
        ModelName::try_new("ThinkPadT14Gen5Intel").unwrap().known(),
        Some(KnownModel::ThinkPadT14Gen5Intel),
    );
    assert_eq!(
        ModelName::try_new("rpi3B").unwrap().known(),
        Some(KnownModel::Rpi3B),
    );
}

#[test]
fn model_name_known_returns_none_for_unknown_string() {
    assert_eq!(
        ModelName::try_new("UnknownLaptop9000").unwrap().known(),
        None,
    );
}

#[test]
fn github_id_round_trip() {
    let id = GithubId::try_new("LiGoldragon").unwrap();
    assert_eq!(id.as_str(), "LiGoldragon");
}

#[test]
fn domain_name_round_trip() {
    let domain = DomainName::try_new("example.test").unwrap();
    assert_eq!(domain.as_str(), "example.test");
}

#[test]
fn criome_domain_name_for_node_renders_dotted_form() {
    let cluster = ClusterName::try_new("goldragon").unwrap();
    let node = NodeName::try_new("ouranos").unwrap();
    let cluster_domain = ClusterDomain::try_new("criome").unwrap();
    let domain = CriomeDomainName::for_node(&node, &cluster, &cluster_domain);
    assert_eq!(domain.as_str(), "ouranos.goldragon.criome");
}

#[test]
fn criome_domain_name_nix_subdomain_prefixes_nix() {
    let cluster = ClusterName::try_new("goldragon").unwrap();
    let node = NodeName::try_new("prometheus").unwrap();
    let cluster_domain = ClusterDomain::try_new("criome").unwrap();
    let domain = CriomeDomainName::for_node(&node, &cluster, &cluster_domain);
    assert_eq!(
        domain.nix_subdomain().as_str(),
        "nix.prometheus.goldragon.criome",
    );
}

#[test]
fn keygrip_accepts_40_hex_chars_normalised_uppercase() {
    let key = Keygrip::try_new("abcdef0123456789abcdef0123456789abcdef01").unwrap();
    assert_eq!(key.as_str(), "ABCDEF0123456789ABCDEF0123456789ABCDEF01");
}

#[test]
fn keygrip_rejects_wrong_length() {
    assert!(Keygrip::try_new("ABCDEF").is_err());
    assert!(Keygrip::try_new("ABCDEF0123456789ABCDEF0123456789ABCDEF0100").is_err());
}

#[test]
fn keygrip_rejects_non_hex_chars() {
    let error = Keygrip::try_new("XYZ_EF0123456789ABCDEF0123456789ABCDEF01").unwrap_err();
    assert!(error.to_string().contains("keygrip"));
}
