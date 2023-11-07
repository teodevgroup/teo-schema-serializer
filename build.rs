extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use std::str::FromStr;


fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR env var is not defined"));

    let profile = env::var("PROFILE")
        .expect("PROFILE env var is not defined");

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    cbindgen::generate_with_config(&crate_dir, config)
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from_str(&format!("target/{profile}")).unwrap().join("teo_schema_serializer.h"));
}