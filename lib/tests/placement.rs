//! Wire-format witnesses for placement types.
//!
//! Confirms the `NexusVerb` (head-identifier sum) and `NotaRecord`
//! derives on `NodePlacement` and its variant payloads round-trip
//! through nota text. This is the contract `NodeProposal.placement`
//! relies on once proposals start authoring placement directly.
//!
//! Spec: `reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md`
//! §P1.1, slice 2b.

use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{NodeName, UserName};
use horizon_lib::placement::{
    Contained, ContainerResources, ContainmentSubstrate, Metal, NodePlacement, UserNamespacePolicy,
};
use horizon_lib::species::{Arch, MotherBoard};
use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

fn round_trip<T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug>(value: T) {
    let mut encoder = Encoder::nota();
    value.encode(&mut encoder).unwrap();
    let text = encoder.into_string();
    let mut decoder = Decoder::nota(&text);
    let recovered = T::decode(&mut decoder).unwrap();
    assert_eq!(value, recovered, "round-trip mismatch via text {text:?}");
}

#[test]
fn metal_placement_round_trips() {
    round_trip(NodePlacement::Metal(Metal {
        arch: Arch::X86_64,
        model: None,
        motherboard: Some(MotherBoard::Ondyfaind),
        ram_gb: Some(32),
    }));
}

#[test]
fn contained_placement_with_pick_policy_round_trips() {
    round_trip(NodePlacement::Contained(Contained {
        host: NodeName::try_new("ouranos").unwrap(),
        substrate: ContainmentSubstrate::NixosContainer,
        resources: ContainerResources { cores: 2, ram_gb: 8 },
        network: None,
        state: None,
        trust: Magnitude::Max.ladder(),
        user_namespace_policy: UserNamespacePolicy::PrivateUsersPick {},
        super_user: Some(UserName::try_new("li").unwrap()),
    }));
}

#[test]
fn contained_placement_with_host_root_mapping_round_trips() {
    round_trip(NodePlacement::Contained(Contained {
        host: NodeName::try_new("hyacinth").unwrap(),
        substrate: ContainmentSubstrate::MicroVm,
        resources: ContainerResources { cores: 4, ram_gb: 16 },
        network: None,
        state: None,
        trust: Magnitude::Medium.ladder(),
        user_namespace_policy: UserNamespacePolicy::HostRootMappingAllowed {
            reason: "legacy workload needs uid:gid pass-through".to_string(),
            approved_by: UserName::try_new("li").unwrap(),
        },
        super_user: None,
    }));
}
