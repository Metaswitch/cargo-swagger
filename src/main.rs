#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]
//! @@@ crate docs.

extern crate docopt;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::{self, File};
use std::io::Read;
use std::process::{self, Command};

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

pub use errors::*;

const USAGE: &str = r"
Generate a Rust crate from a swagger spec using swagger-codegen

Usage:
    cargo swagger <spec-path> <output-path>
    cargo swagger (-h | --help)
    cargo swagger (-V | --version)

Options:
    -h --help                   Show this help page.
    -V --version                Show version.

Requires Docker to be installed.
";

const RUST_GEN_CONTAINER: &str = "swaggerapi/swagger-codegen-cli:latest";

/// Docopts input args.
#[derive(Debug, Deserialize)]
struct Args {
    arg_spec_path: String,
    arg_output_path: String,
    flag_version: bool,
}

fn main() {
    let args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize::<Args>())
        .unwrap_or_else(|err| err.exit());

    if args.flag_version {
        println!("cargo-swagger version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let spec_path = fs::canonicalize(args.arg_spec_path)
        .chain_err(|| "Invalid spec-path")
        .unwrap();
    fs::create_dir_all(&args.arg_output_path)
        .chain_err(|| "Failed to create output directory")
        .unwrap();
    let output_dir = fs::canonicalize(args.arg_output_path)
        .chain_err(|| "Invalid output-path")
        .unwrap();

    // Check that docker is installed? And that we have (or can pull) the requisite container.

    // Copy API doc into output dir
    let output_spec_path = output_dir.join("api.yaml");
    fs::copy(&spec_path, &output_spec_path).unwrap();

    // Parse YAML and extract name
    let crate_name = {
        let mut spec_file = File::open(&spec_path).unwrap();
        let mut spec_contents = String::new();
        spec_file.read_to_string(&mut spec_contents).unwrap();
        let swagger_spec = serde_yaml::from_str::<serde_yaml::Value>(&spec_contents).unwrap();

        swagger_spec["info"]["title"].as_str().unwrap().to_owned()
    };

    let output = Command::new("docker")
        .args(&[
            "run",
            "-v",
            &format!("{}:{}", output_dir.to_string_lossy(), "/tmp/swagger"),
            RUST_GEN_CONTAINER,
            "generate",
            "--lang",
            "rust-server",
            format!("-DpackageName={}", crate_name).as_str(),
            "--input-spec",
            "/tmp/swagger/api.yaml",
            "--output",
            "/tmp/swagger",
        ])
        .output()
        .chain_err(|| "Failed to run generate command")
        .unwrap();

    if !output.status.success() {
        eprintln!(
            "Cargo swagger failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        process::exit(1);
    }
}
