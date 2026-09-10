use std::process::Command;

use datom_codec::Datomizable;
use horizon_lib::*;
use protos::{Protosizable, Textualizable};

fn text(value: &str) -> String {
    value.to_owned()
}

#[test]
fn composer_materializes_one_validated_definition_from_two_explicit_files() {
    let directory = std::env::temp_dir().join(format!("horizon-compose-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("temporary directory");
    let configuration_path = directory.join("configuration.datom");
    let cluster_path = directory.join("cluster-definition.datom");
    let configuration = HorizonConfiguration {
        generic_nodes: Vec::new(),
        domain_configuration: DomainConfiguration {
            string: text("criome"),
            domain_name_vector: vec![text("goldragon.criome.net")],
        },
    };
    let cluster = ClusterDefinition {
        cluster_name: text("goldragon"),
        cluster_nodes: Vec::new(),
        generic_node_names: Vec::new(),
        users: Vec::new(),
        domains: Vec::new(),
        cluster_trust: ClusterTrust {
            magnitude: Magnitude::Max,
            cluster_trust_entry_vector: Vec::new(),
            node_trust_entry_vector: Vec::new(),
            user_trust_entry_vector: Vec::new(),
        },
    };
    std::fs::write(
        &configuration_path,
        configuration.datomize(vec![]).protosize().textualize(),
    )
    .expect("configuration fixture");
    std::fs::write(
        &cluster_path,
        cluster.datomize(vec![]).protosize().textualize(),
    )
    .expect("cluster fixture");

    let request = format!(
        "Compose.{{ {} {} }}",
        configuration_path.display(),
        cluster_path.display()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_horizon-compose"))
        .arg(request)
        .output()
        .expect("composer starts");
    std::fs::remove_dir_all(&directory).expect("temporary directory cleanup");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = HorizonDefinition {
        horizon_configuration: configuration,
        cluster_definition: cluster,
    }
    .datomize(vec![])
    .protosize()
    .textualize();
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 output"),
        format!("{expected}\n")
    );
}
