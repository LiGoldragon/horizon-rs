//! Tests for `Magnitude`.

use horizon_lib::magnitude::{AtLeast, Magnitude};

#[test]
fn ladder_monotonic() {
    assert_eq!(Magnitude::None.ladder(), AtLeast { min: false, med: false, max: false });
    assert_eq!(Magnitude::Min.ladder(), AtLeast { min: true, med: false, max: false });
    assert_eq!(Magnitude::Med.ladder(), AtLeast { min: true, med: true, max: false });
    assert_eq!(Magnitude::Max.ladder(), AtLeast { min: true, med: true, max: true });
}

#[test]
fn floor_picks_lower() {
    assert_eq!(Magnitude::Max.floor(Magnitude::Med), Magnitude::Med);
    assert_eq!(Magnitude::Min.floor(Magnitude::Max), Magnitude::Min);
}

#[test]
fn at_least_compares() {
    assert!(Magnitude::Med.at_least(Magnitude::Min));
    assert!(Magnitude::Med.at_least(Magnitude::Med));
    assert!(!Magnitude::Med.at_least(Magnitude::Max));
}
