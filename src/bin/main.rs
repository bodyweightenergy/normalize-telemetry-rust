use serde_json::*;

use std::fs::*;
use std::io::*;

use json_normalizer::normalize::normalize_map;

fn main() {
    println!("Starting");

    // Read input file
    let file = File::open("C:/git/rust/json_normalizer/data/input.json")
        .expect("error opening input file");
    let reader = BufReader::new(file);
    let s_reader: Value = from_reader(reader).unwrap();

    // Only normalize if valid JSON object
    match s_reader {
        Value::Object(o) => {
            // Normalize input
            let normalized: Value = Value::Object(normalize_map(&o));

            println!("Input: {:?}", o);
            println!("");
            println!("Normalized: {:?}", normalized);

            // Output normalized JSON to file
            let output_file = File::create("C:/git/rust/json_normalizer/data/output.json")
                .expect("Could not create output file.");
            let writer = BufWriter::new(output_file);
            to_writer_pretty(writer, &normalized).expect("failed to serialize to output");
        }
        _ => println!("Not an object: {:?}", s_reader),
    };
}
