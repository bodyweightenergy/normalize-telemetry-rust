/*!
 * Provides methods for JSON manipulation
 *
 * # Purpose
 *
 * To prepare incoming log data to be inserted into Elastisearch, JSON keys must be appended
 * with their value's type (e.g. "str", "num", etc), then must be flattened where there is only
 * one root object containing value items.
 *
 * So for example, the following input JSON:
 * ```
 * {
 *     "name": "john",
 *     "age": 28,
 *     "married": true,
 *     "spouse": {
 *         "name": "maggie",
 *         "age": 27
 *     },
 *     "children": [
 *         { "name": "brad", "age": 4 },
 *         { "name": "julie", "age": 3 }
 *     ]
 * }
 * ```
 *
 * Will be normalized then flattened into:
 *
 * ```
 * {
 *     "str_name": "john",
 *     "num_age": 28,
 *     "bool_married": true,
 *     "obj_spouse_str_name": "maggie",
 *     "obj_spouse_num_age": 27,
 *     "arr_children_0_str_name": "brad",
 *     "arr_children_0_num_age": 4,
 *     "arr_children_1_str_name": "julie",
 *     "arr_children_1_num_age": 3,
 * }
 * ```
 */

pub mod normalize {

    use serde_json::*;

    /// Normalizes a JSON Object
    pub fn normalize_map(map: &Map<String, Value>) -> Map<String, Value> {
        let mut r = Map::default();
        for p in map {
            let n = normalize(&p);
            r.insert(n.0, n.1);
        }

        r
    }

    /// Normalizes a JSON key-value-pair
    pub fn normalize(pair: &(&String, &Value)) -> (String, Value) {
        match &pair.1 {
            Value::Bool(b) => (format!("bool_{}", pair.0), json!(*b)),
            Value::Number(n) => (format!("num_{}", pair.0), json!(n.clone())),
            Value::String(s) => (format!("str_{}", pair.0), json!(s.clone())),
            Value::Null => (format!("str_{}", pair.0), Value::Null),
            Value::Array(v) => (format!("arr_{}", pair.0), json!(v.clone())),
            Value::Object(m) => {
                let mut m_r = Map::default();
                for p in m {
                    let n = normalize(&p);
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
            let mut input = Map::new();

            input.entry("item1").or_insert(json!(0));
            input.entry("item2").or_insert(json!(10));
            input.entry("item3").or_insert(json!(-5.0));

            println!("input = {:?}", input);

            // Act
            let normalized = normalize_map(&input);
            println!("normalized = {:?}", normalized);

            // Assert
            assert_eq!(normalized["num_item1"], 0);
            assert_eq!(normalized["num_item2"], 10);
            assert_eq!(normalized["num_item3"], -5.0);
        }

        #[test]
        fn test_string() {
            // Arrange
            let mut input = Map::new();

            input.entry("item1").or_insert(json!(""));
            input.entry("item2").or_insert(json!("hello"));
            input.entry("item3").or_insert(json!("unicode: G😝©[_ǅƭÛ"));

            println!("input = {:?}", input);

            // Act
            let normalized = normalize_map(&input);
            println!("normalized = {:?}", normalized);

            // Assert
            assert_eq!(normalized["str_item1"], "");
            assert_eq!(normalized["str_item2"], "hello");
            assert_eq!(normalized["str_item3"], "unicode: G😝©[_ǅƭÛ");
        }

        #[test]
        fn test_bool() {
            // Arrange
            let mut input = Map::new();

            input.entry("true").or_insert(json!(true));
            input.entry("false").or_insert(json!(false));

            println!("input = {:?}", input);

            // Act
            let normalized = normalize_map(&input);
            println!("normalized = {:?}", normalized);

            // Assert
            assert_eq!(normalized["bool_true"], true);
            assert_eq!(normalized["bool_false"], false);
        }

        #[test]
        fn test_array() {
            // Arrange
            let mut input = Map::new();

            input.entry("item1").or_insert(json!(vec![0, 1, 2]));
            input
                .entry("item2")
                .or_insert(json!(vec![true, false, true]));
            input.entry("item3").or_insert(json!(vec!["0", "1", "2"]));
            input.entry("item4").or_insert(json!(Vec::<i32>::new()));

            println!("input = {:?}", input);

            // Act
            let normalized = normalize_map(&input);
            println!("normalized = {:?}", normalized);

            // Assert
            assert_eq!(normalized["arr_item1"], json!(vec![0, 1, 2]));
            assert_eq!(normalized["arr_item2"], json!(vec![true, false, true]));
            assert_eq!(normalized["arr_item3"], json!(vec!["0", "1", "2"]));
            assert_eq!(normalized["arr_item4"], json!(Vec::<i32>::new()));
        }

        #[test]
        fn test_object() {
            // Arrange
            let mut input = Map::new();

            input.entry("item1").or_insert(json!({
                "item1": true,
                "item2": 10.0,
                "item3": "hello ǰF🙀😪ğ♆",
                "array1": [ 0, 1, 2 ],
            }));

            println!("input = {:?}", input);

            // Act
            let normalized = normalize_map(&input);
            println!("normalized = {:?}", normalized);

            // Assert
            assert_eq!(
                normalized["obj_item1"],
                json!({
                    "bool_item1": true,
                    "num_item2": 10.0,
                    "str_item3": "hello ǰF🙀😪ğ♆",
                    "arr_array1": [ 0, 1, 2 ],
                })
            );
        }
    }
}

pub mod flatten {

    use serde_json::*;

    pub fn flatten(input: &Map<String, Value>) -> Map<String, Value> {
        let mut r = Map::new();

        for (k, v) in input {
            match v {
                Value::Bool(b) => {
                    r.entry(*k).or_insert(json!(*b));
                }
                Value::Number(n) => {
                    r.entry(*k).or_insert(json!(*n));
                }
                Value::String(s) => {
                    r.entry(*k).or_insert(json!(*s));
                }
                Value::Array(a) => {
                    for (i, j) in a.iter().enumerate() {
                        match j {
                            Value::Bool(b) => {
                                r.entry(format!("{}_{}_bool", *k, i)).or_insert(json!(*b));
                            }
                            Value::Number(n) => {
                                r.entry(format!("{}_{}_num", *k, i)).or_insert(json!(*n));
                            }
                            Value::String(s) => {
                                r.entry(format!("{}_{}_str", *k, i)).or_insert(json!(*s));
                            }
                            Value::Array(a) => {
                                r.entry(format!("{}_{}_arr", *k, i)).or_insert(json!(a));
                            }
                            Value::Object(m) => {
                                r.entry(format!("{}_{}_obj", *k, i)).or_insert(json!(m));
                            }
                            Value::Null => { /* ignore null case */ }
                        }
                    }
                }
                Value::Object(map) => {
                    let map_flat = flatten(&map);
                    for (key, value) in map_flat {
                        r.entry(format!("{}_{}", k, key)).or_insert(value);
                    }
                }
                Value::Null => { /* ignore null case */ }
            }
        }

        r
    }

    #[cfg(tests)]
    mod tests {

    }

}