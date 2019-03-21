use serde_json::*;
use std::io::{self, Read, Write};

use json_normalizer::normalize_flatten;

fn main() -> io::Result<()> {
    eprintln!("Telemetry Normalizer, started.");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input_json: Value = from_str(&buffer[..])?;
    let processed: Value = json!(normalize_flatten(
        &input_json
            .as_object()
            .expect("input JSON is not a valid object")
    ));

    buffer = to_string_pretty(&processed)?;
    io::stdout().write_all(&buffer.as_bytes())?;

    eprintln!("Processing complete.");
    Ok(())
}
