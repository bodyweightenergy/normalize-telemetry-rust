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
 * ```json
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
 * ```json
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

pub mod flatten;
pub mod normalize;

use flatten::flatten_map;
use normalize::normalize_map;
use serde_json::{Map, Value};

pub fn get_type_str(value: &Value) -> String {
    match value {
        Value::Bool(_) => "bool".to_string(),
        Value::Number(_) => "num".to_string(),
        Value::String(_) => "str".to_string(),
        Value::Null => "null".to_string(),
        Value::Array(_) => "arr".to_string(),
        Value::Object(_) => "obj".to_string(),
    }
}

pub fn normalize_flatten(json: &Map<String, Value>) -> Map<String, Value> {
    flatten_map(&normalize_map(json))
}

#[cfg(test)]
mod tests {
    use super::flatten::*;
    use super::normalize::*;
    use super::*;
    use serde_json::*;

    #[test]
    fn test_all() {
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
                        1.0,
                        "dancing",
                    ]
                }
            ]
        });

        // Act
        let normalized = normalize_map(&input.as_object().unwrap());
        let flattened = flatten_map(&normalized);

        let actual = json!(normalize_flatten(&input.as_object().unwrap()));

        // Assert
        let expected = json!({
            "str_name": "john",
            "num_age": 28,
            "bool_married": true,
            "null_birthday": null,
            "obj_spouse_str_name": "maggie",
            "obj_spouse_num_age": 27,
            "arr_children_0_str_name": "brad",
            "arr_children_0_num_age": 4,
            "arr_children_1_str_name": "julie",
            "arr_children_1_num_age": 3,
            "arr_children_1_arr_hobbies_0_num": 1.0,
            "arr_children_1_arr_hobbies_1_str": "dancing",
        });
        println!("Normalized:");
        println!("{}", to_string_pretty(&normalized).unwrap());
        println!("");
        println!("Flattened:");
        println!("{}", to_string_pretty(&flattened).unwrap());
        println!("");
        println!("Expected:");
        println!("{}", to_string_pretty(&expected).unwrap());

        assert!(
            expected.eq(&json!(&flattened)),
            "object not normalized/flattened correctly."
        );
        assert!(
            expected.eq(&actual),
            "normalized_flatten() function output does not match the separate parts."
        );
    }

}
