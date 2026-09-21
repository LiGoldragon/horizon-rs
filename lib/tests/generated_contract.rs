use datomic::{Datomic, TextEdge};
use horizon_lib::{magnitude::Magnitude, species::MachineSpecies};
use protos::Text;

#[test]
fn generated_d3_anatomy_round_trips_the_hand_owned_magnitude() {
    let text = Magnitude::Large.textualize();
    assert_eq!(text.as_ref(), "Large");
    assert_eq!(
        Text::<Magnitude>::from(text.as_ref())
            .embody()
            .expect("generated Datomic embodiment")
            .textualize()
            .as_ref(),
        text.as_ref(),
    );
}

#[test]
fn generated_d3_anatomy_round_trips_hand_owned_species() {
    let text = MachineSpecies::Pod.textualize();
    assert_eq!(text.as_ref(), "Pod");
    assert!(matches!(
        Text::<MachineSpecies>::from(text.as_ref())
            .embody()
            .expect("Datomic embodiment"),
        MachineSpecies::Pod
    ));
}
