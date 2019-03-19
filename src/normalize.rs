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
    fn test_basic() {

        // Arrange
        let mut input: Map<String, Value> = Map::new();
        input.insert("number_item".to_string(), Value::Number(10.into()));
        input.insert("string_item".to_string(), Value::String("value".to_string()));
        input.insert("bool_item".to_string(), Value::Bool(true));

        // Act
        let normalized = normalize_map(&input);

        // Assert
        match normalized.get("number_item").unwrap() {
            Value::Number(n) => assert_eq!(n.as_i64().unwrap(), 10),
            _ => assert!(false)
        }

        match normalized.get("string_item").unwrap() {
            Value::String(s) => assert_eq!(s, "value"),
            _ => assert!(false)
        }

        match normalized.get("bool_item").unwrap() {
            Value::Bool(b) => assert_eq!(*b, true),
            _ => assert!(false)
        }
    }
}