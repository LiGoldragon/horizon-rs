use std::process::Command;

use datom_codec::Textualizable;
use horizon_lib::*;

fn text(value: &str) -> protos::Text {
    protos::Text::try_from(value).expect("fixture text")
}

#[test]
fn composer_materializes_one_validated_definition_from_two_explicit_files() {
    let directory = std::env::temp_dir().join(format!("horizon-compose-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("temporary directory");
    let configuration_path = directory.join("configuration.datom");
    let cluster_path = directory.join("cluster-definition.datom");
    let configuration = HorizonConfiguration(
        Vec::new(),
        DomainConfiguration(text("criome"), vec![text("goldragon.criome.net")]),
    );
    let cluster = ClusterDefinition(
        text("goldragon"),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        ClusterTrust(Magnitude::Max, Vec::new(), Vec::new(), Vec::new()),
    );
    std::fs::write(&configuration_path, configuration.textualize()).expect("configuration fixture");
    std::fs::write(&cluster_path, cluster.textualize()).expect("cluster fixture");

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
    let expected = HorizonDefinition(configuration, cluster).textualize();
    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 output"),
        format!("{expected}\n")
    );
}
