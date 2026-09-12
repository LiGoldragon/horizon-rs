//! Deterministically materialize one HorizonDefinition from two typed inputs.

use std::process::ExitCode;

use datom_codec::Datomizable;
use horizon_lib::{
    ClusterDefinition, Composing, CompositionCommand, DatomDecoding, HorizonConfiguration,
};
use protos::{Protosizable, Textualizable};

fn main() -> ExitCode {
    let Some(request) = std::env::args().nth(1) else {
        eprintln!("error: expected Compose.{{ configuration-path cluster-path }}");
        return ExitCode::from(2);
    };
    if std::env::args().nth(2).is_some() {
        eprintln!("error: horizon-compose accepts exactly one Datom request");
        return ExitCode::from(2);
    }
    let request = match CompositionCommand::decode(&request) {
        Ok(request) => request,
        Err(error) => {
            eprintln!("error: parse composition request: {error}");
            return ExitCode::from(2);
        }
    };
    let CompositionCommand::Compose(request) = request;
    let configuration = match std::fs::read_to_string(&request.first_string) {
        Ok(text) => match HorizonConfiguration::decode(&text) {
            Ok(configuration) => configuration,
            Err(error) => {
                eprintln!("error: parse HorizonConfiguration: {error}");
                return ExitCode::from(1);
            }
        },
        Err(error) => {
            eprintln!("error: read HorizonConfiguration: {error}");
            return ExitCode::from(2);
        }
    };
    let cluster = match std::fs::read_to_string(&request.second_string) {
        Ok(text) => match ClusterDefinition::decode(&text) {
            Ok(cluster) => cluster,
            Err(error) => {
                eprintln!("error: parse ClusterDefinition: {error}");
                return ExitCode::from(1);
            }
        },
        Err(error) => {
            eprintln!("error: read ClusterDefinition: {error}");
            return ExitCode::from(2);
        }
    };
    match configuration.compose(cluster) {
        Ok(definition) => println!("{}", definition.datomize(vec![]).protosize().textualize()),
        Err(error) => {
            eprintln!("error: compose HorizonDefinition: {error}");
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
