use std::fs::File;
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;

#[derive(Deserialize, Serialize)]
struct Source {
    pub name: String,
    pub language: String,
    pub version: String,
    pub url: String,
    pub nsfw: bool,
}

// Copy the version from Cargo.toml to res/source.json
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the source.json file
    let mut source_content = String::new();
    let mut source_file = File::open("res/source.json")?;
    source_file.read_to_string(&mut source_content)?;

    // Deserialize the source.json file
    let mut source: Source = serde_json::from_str(&source_content)?;

    // Update the version
    source.version = std::env::var("CARGO_PKG_VERSION")?;

    // Serialize the source.json file with an indent of 4 spaces
    let mut source_content_vec = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut serializer = Serializer::with_formatter(&mut source_content_vec, formatter);
    source.serialize(&mut serializer)?;
    let source_content = String::from_utf8(source_content_vec)?;

    // Write the source.json file
    let mut source_file = File::create("res/source.json")?;
    source_file.write_all(source_content.as_bytes())?;

    Ok(())
}
