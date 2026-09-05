//! Deterministically materialize one HorizonDefinition from two typed inputs.

use std::process::ExitCode;

use datom_codec::Textualizable;

fn main() -> ExitCode {
    let Some(request) = std::env::args().nth(1) else {
        eprintln!("error: expected Compose.{{ configuration-path cluster-path }}");
        return ExitCode::from(2);
    };
    if std::env::args().nth(2).is_some() {
        eprintln!("error: horizon-compose accepts exactly one Datom request");
        return ExitCode::from(2);
    }
    let request = match horizon_lib::decode_composition_request(&request) {
        Ok(request) => request,
        Err(error) => {
            eprintln!("error: parse composition request: {error}");
            return ExitCode::from(2);
        }
    };
    let horizon_lib::CompositionCommand::Compose(request) = request;
    let configuration = match std::fs::read_to_string(request.0.as_ref()) {
        Ok(text) => match horizon_lib::decode_configuration(&text) {
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
    let cluster = match std::fs::read_to_string(request.1.as_ref()) {
        Ok(text) => match horizon_lib::decode_cluster(&text) {
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
    match horizon_lib::compose(configuration, cluster) {
        Ok(definition) => print!("{}\n", definition.textualize()),
        Err(error) => {
            eprintln!("error: compose HorizonDefinition: {error}");
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
