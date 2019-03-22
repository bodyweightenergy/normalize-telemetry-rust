use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::{json, Map, Value};
use std::iter;

pub fn rand_json_value<R: Rng + ?Sized>(rng: &mut R, count: i32, depth: i32) -> Value {
    let ceiling: i32 = if depth > 0 { 6 } else { 4 };
    match rng.gen_range(0, ceiling) {
        0 => Value::Bool(rng.gen()),
        1 => json!(rng.gen::<f64>()),
        3 => json!(rand_str(rng, 20)),
        4 => json!(rand_json_array(rng, count, depth - 1)),
        5 => json!(rand_json_obj(rng, count, depth - 1)),
        _ => Value::Null,
    }
}

/// Generates a JSON array
pub fn rand_json_array<R: Rng + ?Sized>(rng: &mut R, count: i32, depth: i32) -> Vec<Value> {
    let mut result: Vec<Value> = vec![];
    for _ in 0..20 {
        result.push(rand_json_value(rng, count, depth - 1));
    }
    result
}

/// Generate a JSON object
pub fn rand_json_obj<R: Rng + ?Sized>(rng: &mut R, count: i32, depth: i32) -> Map<String, Value> {
    let mut result = Map::new();
    for _ in 0..count {
        let key = rand_str(rng, 5);
        result
            .entry(key)
            .or_insert(rand_json_value(rng, count, depth - 1));
    }
    result
}

pub fn rand_str<R: Rng + ?Sized>(rng: &mut R, len: usize) -> String {
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len)
        .collect();
    chars
}
