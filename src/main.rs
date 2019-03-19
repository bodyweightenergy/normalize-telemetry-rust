use serde_json::*;

use std::fs::*;
use std::io::*;

fn main() {
    println!("Starting");

    let file = File::open("C:/git/rust/json_normalizer/data/input.json")
        .expect("error opening input file");
    let reader = BufReader::new(file);
    let s_reader: Value = from_reader(reader).unwrap();

    match s_reader {
        Value::Object(o) => {
            let normalized: Value = Value::Object(normalize_map(&o));

            println!("Input: {:?}", o);
            println!("");
            println!("Normalized: {:?}", normalized);

            let output_file = File::create("C:/git/rust/json_normalizer/data/output.json")
                .expect("Could not create output file.");
            let writer = BufWriter::new(output_file);
            to_writer_pretty(writer, &normalized).expect("failed to serialize to output");
        }
        _ => println!("Not an object: {:?}", s_reader),
    };
}

fn normalize_map(map: &Map<String, Value>) -> Map<String, Value> {
    let mut r = Map::default();
    for p in map {
        let n = normalize(&p);
        r.insert(n.0, n.1);
    }

    r
}

fn normalize(pair: &(&String, &Value)) -> (String, Value) {
    match &pair.1 {
        Value::Bool(b) => (format!("bool_{}", pair.0), Value::Bool(*b)),
        Value::Number(n) => (format!("num_{}", pair.0), Value::Number(n.clone())),
        Value::String(s) => (format!("str_{}", pair.0), Value::String(s.clone())),
        Value::Null => (format!("str_{}", pair.0), Value::Null),
        Value::Array(v) => (format!("arr_{}", pair.0), Value::Array(v.clone())),
        Value::Object(m) => {
            let mut m_r = Map::default();
            for p in m {
                let n = normalize(&p);
                m_r.insert(n.0, n.1);
            }
            (format!("obj_{}", pair.0), Value::Object(m_r))
        }
    }
}
