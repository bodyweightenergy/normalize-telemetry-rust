use serde_json::*;

pub fn flatten_map(input: &Map<String, Value>) -> Map<String, Value> {
    let mut r = Map::new();

    for (k1, v1) in input {
        match v1 {
            Value::Bool(b) => {
                r.entry(k1.clone()).or_insert(json!(*b));
            }
            Value::Number(n) => {
                r.entry(k1.clone()).or_insert(json!(*n));
            }
            Value::String(s) => {
                r.entry(k1.clone()).or_insert(json!(*s));
            }
            Value::Null => {
                r.entry(k1.clone()).or_insert(Value::Null);
            }
            Value::Array(a) => {
                for (i, j) in a.iter().enumerate() {
                    match j {
                        Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Null => {
                            r.entry(format!("{}_{}_{}", *k1, i, get_type_str(j)))
                                .or_insert(j.clone());
                        }
                        Value::Array(a) => {
                            r.entry(format!("{}_{}_arr", *k1, i))
                                .or_insert(json!(flatten_array(a)));
                        }
                        Value::Object(m) => {
                            let flat_map = flatten_map(m);
                            for (ak, av) in flat_map {
                                r.entry(format!("{}_{}_{}", *k1, i, ak))
                                    .or_insert(json!(av));
                            }
                        }
                    }
                }
            }
            Value::Object(map) => {
                let map_flat = flatten_map(&map);
                for (k2, v2) in map_flat {
                    r.entry(format!("{}_{}", k1, k2)).or_insert(v2);
                }
            }
        }
    }

    r
}

use super::get_type_str;

pub fn flatten_array(a: &Vec<Value>) -> Map<String, Value> {
    let mut r = Map::new();
    for (i, v) in a.iter().enumerate() {
        r.entry(format!("{}_{}", i, get_type_str(v)))
            .or_insert(v.clone());
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_string_pretty, Value};

    #[test]
    fn test_basic() {
        // Arrange
        let input = json!({
           "name": "john",
           "age": 28,
           "married": true,
           "birthday": null,
            "spouse": {
                "name": "maggie",
                "age": 27
            },
            "children": [
                { "name": "brad", "age": 4 },
                {
                    "name": "julie",
                    "age": 3,
                    "hobbies": [
                        "soccer",
                        "dancing",
                    ]
                }
            ]
        });

        // Act
        let flattened = Value::Object(flatten_map(&input.as_object().unwrap()));

        // Assert
        let expected = json!({
            "name": "john",
            "age": 28,
            "married": true,
            "birthday": null,
            "spouse_name": "maggie",
            "spouse_age": 27,
            "children_0_name": "brad",
            "children_0_age": 4,
            "children_1_name": "julie",
            "children_1_age": 3,
            "children_1_hobbies_0_str": "soccer",
            "children_1_hobbies_1_str": "dancing",
        });

        println!("Actual:");
        println!("{}", to_string_pretty(&flattened).unwrap());
        println!("");
        println!("Expected:");
        println!("{}", to_string_pretty(&expected).unwrap());

        assert!(
            expected.eq(&flattened),
            "flattened input does not match expected JSON."
        );
    }
}
