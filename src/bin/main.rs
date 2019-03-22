use clap::{App, Arg, SubCommand};
use rand::thread_rng;
use serde_json::*;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

use json_normalizer::{normalize_flatten, utils};

fn main() -> io::Result<()> {
    let matches = App::new("Telemetry Normalizer")
        .version("0.1")
        .author("Hazim Salem <apklemon@gmail.com>")
        .about("Normalizes keys and flattens JSON payloads")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Path to input JSON file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Path to normalized JSON file")
                .takes_value(true),
        )
        .get_matches();

    let input_path = matches.value_of("input").unwrap_or("input.json");
    let output_path = matches.value_of("output").unwrap_or("output.json");

    println!("Input file: {}", input_path);
    println!("Output file: {}", output_path);

    let input_file = File::open(input_path)?;
    let input: Value = from_reader(BufReader::new(input_file))?;

    let processed = json!(normalize_flatten(
        &input.as_object().expect("Input is not JSON object")
    ));

    let output_writer =
        BufWriter::new(File::create(output_path).expect("Could not create output file."));

    to_writer_pretty(output_writer, &processed)?;

    Ok(())
}
