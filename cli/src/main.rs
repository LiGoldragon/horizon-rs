use horizon_lib::nix::{OutputFile, StructuredAttrs};
use horizon_lib::{
    data::Data,
    horizon::{Data, Horizon},
};
use serde_json;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let struct_attrs: StructuredAttrs = StructuredAttrs::from_cwd();

    let horizon_data = Data::try_from(&struct_attrs);

    let horizon: Horizon = Horizon::try_from(horizon_data);

    let reserialized_data: String = serde_json::to_string(&struct_attrs.attrs.get("horizon-data"))
        .expect("Error Serializing Data");

    let mut output_file: OutputFile =
        OutputFile::try_from(struct_attrs).expect("Error: getting output file");

    output_file
        .write(reserialized_data.as_bytes())
        .expect("Error writing Data");

    Ok(())
}
