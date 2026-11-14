//! The ethos-zero 28 newtypes keep the representation the 0.17.1 type
//! aliases had. The two fixtures were written by horizon-lib 0.17.1
//! (e723ce1, ethos-zero 16 aliases, datom-codec 0.32.2) from one definition
//! that carries every node capability, both virtual machine hosts, a fixed
//! location and every user capability: `definition-0.17.1.rkyv` is its rkyv
//! archive, `definition-0.17.1.datom` its compact Datom text.

use horizon_lib::*;
use rkyv::rancor::Error;

const ARCHIVE: &[u8] = include_bytes!("fixtures/definition-0.17.1.rkyv");

#[test]
fn the_0_17_1_archive_reads_and_archives_back_to_the_same_bytes() {
    let definition =
        rkyv::from_bytes::<HorizonDefinition, Error>(ARCHIVE).expect("read the 0.17.1 archive");
    let bytes = rkyv::to_bytes::<Error>(&definition).expect("archive the definition");
    assert_eq!(bytes.as_slice(), ARCHIVE);
}

#[cfg(feature = "datom")]
mod text {
    use super::*;
    use datom_codec::Datomizable;
    use protos::{Compactable, Protosizable};

    const TEXT: &str = include_str!("fixtures/definition-0.17.1.datom");

    #[test]
    fn the_0_17_1_text_reads_and_archives_to_the_0_17_1_bytes() {
        let definition = HorizonDefinition::decode(TEXT).expect("read the 0.17.1 text");
        let bytes = rkyv::to_bytes::<Error>(&definition).expect("archive the definition");
        assert_eq!(bytes.as_slice(), ARCHIVE);
    }

    #[test]
    fn the_0_17_1_archive_writes_the_0_17_1_text() {
        let definition =
            rkyv::from_bytes::<HorizonDefinition, Error>(ARCHIVE).expect("read the 0.17.1 archive");
        assert_eq!(definition.datomize(vec![]).protosize().compact(), TEXT);
    }
}
