use serde_json::*;

/// Normalizes a JSON Object
pub fn normalize_map(map: &Map<String, Value>) -> Map<String, Value> {
    let mut r = Map::new();
    for p in map {
        let n = normalize_pair(&p);
        r.insert(n.0, n.1);
    }

    r
}

/// Normalizes a JSON key-value-pair
pub fn normalize_pair(pair: &(&String, &Value)) -> (String, Value) {
    match &pair.1 {
        Value::Bool(b) => (format!("bool_{}", pair.0), json!(*b)),
        Value::Number(n) => (format!("num_{}", pair.0), json!(n.clone())),
        Value::String(s) => (format!("str_{}", pair.0), json!(s.clone())),
        Value::Null => (format!("null_{}", pair.0), Value::Null),
        Value::Array(v) => {
            let mut v_norm: Vec<Value> = vec![];
            for i in v {
                match i {
                    Value::Object(m) => v_norm.push(json!(normalize_map(&m))),
                    _ => v_norm.push(i.clone()),
                };
            }
            (format!("arr_{}", pair.0), json!(v_norm))
        }
        Value::Object(m) => {
            let mut m_r = Map::new();
            for p in m {
                let n = normalize_pair(&p);
                m_r.insert(n.0, n.1);
            }
            (format!("obj_{}", pair.0), json!(m_r))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_map;
    use serde_json::*;

    #[test]
    fn test_number() {
        // Arrange
        let input = json!({
            "item1": 0,
            "item2": 10,
            "item3": -5.0
        });

        println!("input = {:?}", input);

        // Act
        let normalized = json!(normalize_map(&input.as_object().unwrap()));
        println!("normalized = {:?}", normalized);

        // Assert
        let expected = json!({
            "num_item1": 0,
            "num_item2": 10,
            "num_item3": -5.0
        });

        assert!(
            expected.eq(&normalized),
            "numbers not normalized correctly."
        );
    }

    #[test]
    fn test_string() {
        // Arrange
        let input = json!({
            "item1": "",
            "item2": "hello",
            "item3": "unicode: G😝©[_ǅƭÛ",
        });

        println!("input = {:?}", input);

        // Act
        let normalized = json!(normalize_map(&input.as_object().unwrap()));
        println!("normalized = {:?}", normalized);

        // Assert
        let expected = json!({
            "str_item1": "",
            "str_item2": "hello",
            "str_item3": "unicode: G😝©[_ǅƭÛ",
        });

        assert!(
            expected.eq(&normalized),
            "strings not normalized correctly."
        );
    }

    #[test]
    fn test_bool() {
        // Arrange
        let input = json!({
            "true_item": true,
            "false_item": false,
        });

        println!("input = {:?}", input);

        // Act
        let normalized = json!(normalize_map(&input.as_object().unwrap()));
        println!("normalized = {:?}", normalized);

        // Assert
        let expected = json!({
            "bool_true_item": true,
            "bool_false_item": false,
        });

        assert!(expected.eq(&normalized), "bools not normalized correctly.");
    }

    #[test]
    fn test_array_basic() {
        // Arrange
        let input = json!({
            "arr1": [0, 1, 2, 3.0, -10.4],
            "arr2": [true, true, false],
            "arr3": ["one", "two", "three"],
            "arr4": [null, null, null]
        });

        println!("input = {:?}", input);

        // Act
        let normalized = json!(normalize_map(&input.as_object().unwrap()));
        println!("normalized = {:?}", normalized);

        // Assert
        let expected = json!({
            "arr_arr1": [0, 1, 2, 3.0, -10.4],
            "arr_arr2": [true, true, false],
            "arr_arr3": ["one", "two", "three"],
            "arr_arr4": [null, null, null]
        });

        assert!(
            expected.eq(&normalized),
            "basic arrays not normalized correctly."
        );
    }

    #[test]
    fn test_array_complex() {
        // Arrange
        let input = json!({
            "arr1": [
                0,
                "1",
                true,
                null,
                [
                    "2", 3, 4.0, false, null
                ],
                {
                    "item1": 1,
                    "item2": "2",
                    "item3": true,
                    "item6": null,
                    "item4": [0, "2", false, null],
                    "item5": { "subitem6": 7 }
                },
            ],
        });

        // Act
        let normalized = json!(normalize_map(&input.as_object().unwrap()));
        println!("Normalized:");
        println!("{}", to_string_pretty(&normalized).unwrap());

        // Assert
        let expected = json!({
            "arr_arr1": [
                0,
                "1",
                true,
                null,
                [
                    "2", 3, 4.0, false, null
                ],
                {
                    "num_item1": 1,
                    "str_item2": "2",
                    "bool_item3": true,
                    "null_item6": null,
                    "arr_item4": [0, "2", false, null],
                    "obj_item5": {
                        "num_subitem6": 7,
                    }
                },
            ],
        });
        println!("Expected:");
        println!("{}", to_string_pretty(&expected).unwrap());

        assert!(
            expected.eq(&normalized),
            "com arrays not normalized correctly."
        );
    }

    #[test]
    fn test_object() {
        // Arrange
        let input = json!({
            "item1": true,
            "item2": 10.0,
            "item3": "hello ǰF🙀😪ğ♆",
            "item4": null,
            "array1": [ 0, 1, 2 ],
            "object1": {
                "item1": true,
                "item2": 10.0,
                "item3": "hello ǰF🙀😪ğ♆",
                "item4": null,
            }
        });

        println!("input = {:?}", input);

        // Act
        let normalized = json!(normalize_map(&input.as_object().unwrap()));
        println!("normalized = {:?}", normalized);

        // Assert
        let expected = json!({
            "bool_item1": true,
            "num_item2": 10.0,
            "str_item3": "hello ǰF🙀😪ğ♆",
            "null_item4": null,
            "arr_array1": [ 0, 1, 2 ],
            "obj_object1": {
                "bool_item1": true,
                "num_item2": 10.0,
                "str_item3": "hello ǰF🙀😪ğ♆",
                "null_item4": null,
            }
        });

        assert!(
            expected.eq(&normalized),
            "complex objects not normalized corrected."
        );
    }
}
