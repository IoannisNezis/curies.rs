use std::collections::HashSet;

use curies::Record;
use extendr_api::prelude::*;

use ::curies::{sources::get_bioregistry_converter, Converter};
use tokio::runtime::Runtime;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

pub struct ConverterR {
    converter: Converter,
    // Getting error when using the converter struct
    // "Error in `value[[3L]]()`: Failed to generate wrapper functions"
    pub name: String,
}

#[extendr]
impl ConverterR {
    fn new() -> Result<Self> {
        // Building from empty converter works
        // let mut converter = Converter::default();
        // let record1 = Record {
        //     prefix: "doid".to_string(),
        //     uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
        //     prefix_synonyms: HashSet::from(["DOID".to_string()]),
        //     uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
        //     pattern: None,
        // };
        // converter.add_record(record1).unwrap();
        // But import using the async function and rt.block_on fails to generate wrapper functions
        let converter = init_converter();
        Ok(Self {
            converter,
            name: "".to_string(),
        })
    }

    fn compress(&self, uri: &str) -> String {
        self.converter.compress(uri).unwrap()
    }
}

fn init_converter() -> Converter {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { get_bioregistry_converter().await.unwrap() })
}

// Macro to generate exports. This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod curiesr;
    fn hello_world;
    impl ConverterR;
}
