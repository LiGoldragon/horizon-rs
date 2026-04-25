//! Tests for `Magnitude`.

use horizon_lib::magnitude::{AtLeast, Magnitude};

#[test]
fn ladder_monotonic() {
    assert_eq!(
        Magnitude::None.ladder(),
        AtLeast { at_least_min: false, at_least_med: false, at_least_large: false, at_least_max: false }
    );
    assert_eq!(
        Magnitude::Min.ladder(),
        AtLeast { at_least_min: true, at_least_med: false, at_least_large: false, at_least_max: false }
    );
    assert_eq!(
        Magnitude::Med.ladder(),
        AtLeast { at_least_min: true, at_least_med: true, at_least_large: false, at_least_max: false }
    );
    assert_eq!(
        Magnitude::Large.ladder(),
        AtLeast { at_least_min: true, at_least_med: true, at_least_large: true, at_least_max: false }
    );
    assert_eq!(
        Magnitude::Max.ladder(),
        AtLeast { at_least_min: true, at_least_med: true, at_least_large: true, at_least_max: true }
    );
}

#[test]
fn floor_picks_lower() {
    assert_eq!(Magnitude::Max.floor(Magnitude::Med), Magnitude::Med);
    assert_eq!(Magnitude::Min.floor(Magnitude::Max), Magnitude::Min);
    assert_eq!(Magnitude::Max.floor(Magnitude::Large), Magnitude::Large);
}

#[test]
fn at_least_compares() {
    assert!(Magnitude::Med.at_least(Magnitude::Min));
    assert!(Magnitude::Med.at_least(Magnitude::Med));
    assert!(!Magnitude::Med.at_least(Magnitude::Large));
    assert!(!Magnitude::Med.at_least(Magnitude::Max));
    assert!(Magnitude::Large.at_least(Magnitude::Med));
    assert!(!Magnitude::Large.at_least(Magnitude::Max));
}
