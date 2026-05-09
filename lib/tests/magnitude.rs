//! Tests for `Magnitude`.

use horizon_lib::magnitude::{AtLeast, Magnitude};

#[test]
fn ladder_monotonic() {
    assert_eq!(
        Magnitude::Zero.ladder(),
        AtLeast { min: false, medium: false, large: false, max: false }
    );
    assert_eq!(
        Magnitude::Min.ladder(),
        AtLeast { min: true, medium: false, large: false, max: false }
    );
    assert_eq!(
        Magnitude::Medium.ladder(),
        AtLeast { min: true, medium: true, large: false, max: false }
    );
    assert_eq!(
        Magnitude::Large.ladder(),
        AtLeast { min: true, medium: true, large: true, max: false }
    );
    assert_eq!(
        Magnitude::Max.ladder(),
        AtLeast { min: true, medium: true, large: true, max: true }
    );
}

#[test]
fn min_picks_lower() {
    assert_eq!(Magnitude::Max.min(Magnitude::Medium), Magnitude::Medium);
    assert_eq!(Magnitude::Min.min(Magnitude::Max), Magnitude::Min);
    assert_eq!(Magnitude::Max.min(Magnitude::Large), Magnitude::Large);
}

#[test]
fn at_least_compares() {
    assert!(Magnitude::Medium.at_least(Magnitude::Min));
    assert!(Magnitude::Medium.at_least(Magnitude::Medium));
    assert!(!Magnitude::Medium.at_least(Magnitude::Large));
    assert!(!Magnitude::Medium.at_least(Magnitude::Max));
    assert!(Magnitude::Large.at_least(Magnitude::Medium));
    assert!(!Magnitude::Large.at_least(Magnitude::Max));
}
