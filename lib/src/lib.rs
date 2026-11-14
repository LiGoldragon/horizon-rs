//! Horizon's typed Datom definition, resolution, and viewpoint projection.

pub mod generated;
pub mod model;
pub mod projection;

pub use generated::horizon::*;
pub use model::*;
pub use projection::*;

#[cfg(test)]
mod portable_tests {
    use super::*;

    #[test]
    fn horizon_definition_crosses_a_portable_archive_without_datom() {
        let definition = HorizonDefinition {
            horizon_configuration: HorizonConfiguration {
                generic_nodes: GenericNodes(vec![]),
                domain_configuration: DomainConfiguration {
                    string: "internal.invalid".into(),
                    domain_name_vector: vec![],
                },
            },
            cluster_definition: ClusterDefinition {
                cluster_name: ClusterName("fixture-cluster".into()),
                cluster_nodes: ClusterNodes(vec![]),
                generic_node_names: GenericNodeNames(vec![]),
                users: Users(vec![]),
                domains: Domains(vec![]),
                cluster_trust: ClusterTrust {
                    magnitude: Magnitude::Zero,
                    cluster_trust_entry_vector: vec![],
                    node_trust_entry_vector: vec![],
                    user_trust_entry_vector: vec![],
                },
            },
        };
        let bytes =
            rkyv::to_bytes::<rkyv::rancor::Error>(&definition).expect("archive Horizon definition");
        let received = bytes.to_vec();
        let restored = rkyv::from_bytes::<HorizonDefinition, rkyv::rancor::Error>(&received)
            .expect("restore Horizon definition from received bytes");
        assert_eq!(restored, definition);
    }
}
