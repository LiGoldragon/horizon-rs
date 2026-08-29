use datomic::{Datomic, TextEdge};
use horizon_lib::{
    address::TapSubnet,
    domain::{DomainConfiguration, InternalDomainSuffix, PublicClusterDomain},
    name::NodeName,
};
use protos::Text;

#[test]
fn validated_node_name_round_trips_through_the_datomic_text_edge() {
    let name = NodeName::try_new("prometheus").expect("valid name");
    let text = name.textualize();
    assert_eq!(text.as_ref(), "prometheus");
    assert_eq!(
        Text::<NodeName>::from(text.as_ref())
            .embody()
            .expect("Datomic embodiment")
            .as_str(),
        "prometheus"
    );
}

#[test]
fn validated_tap_subnet_round_trips_through_the_datomic_text_edge() {
    let subnet = TapSubnet::try_new("169.254.100.0/22").expect("valid subnet");
    let text = subnet.textualize();
    assert_eq!(text.as_ref(), "169.254.100.0/22");
    assert_eq!(
        Text::<TapSubnet>::from(text.as_ref())
            .embody()
            .expect("Datomic embodiment")
            .to_string(),
        "169.254.100.0/22"
    );
}

#[test]
fn domain_configuration_round_trips_as_a_positional_datomic_record() {
    let configuration = DomainConfiguration {
        internal_suffix: InternalDomainSuffix::new("criome"),
        public_cluster_domains: vec![PublicClusterDomain::new("goldragon.criome.net")],
    };
    let text = configuration.textualize();
    assert_eq!(text.as_ref(), "{criome [goldragon.criome.net]}");
    assert_eq!(
        Text::<DomainConfiguration>::from(text.as_ref())
            .embody()
            .expect("Datomic embodiment")
            .textualize()
            .as_ref(),
        text.as_ref()
    );
}
