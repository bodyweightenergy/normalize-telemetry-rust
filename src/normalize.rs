use serde_json::*;

pub fn normalize_map(map: &Map<String, Value>) -> Map<String, Value> {
    let mut r = Map::default();
    for p in map {
        let n = normalize(&p);
        r.insert(n.0, n.1);
    }

    r
}

pub fn normalize(pair: &(&String, &Value)) -> (String, Value) {
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

#[cfg(test)]
mod tests {
    use crate::normalize::normalize_map;
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
}