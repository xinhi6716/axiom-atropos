use serde_json::Value;
pub fn validate(schema: &Value, value: &Value) -> Result<(), String> {
    check(schema, value, "$")
}
fn check(s: &Value, v: &Value, path: &str) -> Result<(), String> {
    if let Some(t) = s.get("type").and_then(Value::as_str) {
        let ok = match t {
            "object" => v.is_object(),
            "array" => v.is_array(),
            "string" => v.is_string(),
            "integer" => {
                v.as_i64().is_some()
                    || v.as_u64().is_some()
                    || v.as_f64().is_some_and(|n| n.fract() == 0.0)
            }
            "number" => v.is_number(),
            "boolean" => v.is_boolean(),
            "null" => v.is_null(),
            _ => true,
        };
        if !ok {
            return Err(format!("{path}: expected {t}"));
        }
    }
    if let Some(n) = v.as_f64() {
        if s["minimum"].as_f64().is_some_and(|m| n < m)
            || s["maximum"].as_f64().is_some_and(|m| n > m)
        {
            return Err(format!("{path}: outside allowed range"));
        }
    }
    if let Some(e) = s.get("enum").and_then(Value::as_array) {
        if !e.contains(v) {
            return Err(format!("{path}: not in enum"));
        }
    }
    if let Some(obj) = v.as_object() {
        if let Some(req) = s.get("required").and_then(Value::as_array) {
            for key in req.iter().filter_map(Value::as_str) {
                if !obj.contains_key(key) {
                    return Err(format!("{path}.{key}: required"));
                }
            }
        }
        let props = s.get("properties").and_then(Value::as_object);
        for (key, val) in obj {
            let sub = format!("{path}.{key}");
            if let Some(schema) = props.and_then(|p| p.get(key)) {
                check(schema, val, &sub)?;
            } else if s.get("additionalProperties") == Some(&Value::Bool(false)) {
                return Err(format!("{sub}: additional property"));
            }
        }
    }
    if let (Some(items), Some(arr)) = (s.get("items"), v.as_array()) {
        for (i, item) in arr.iter().enumerate() {
            check(items, item, &format!("{path}[{i}]"))?;
        }
    }
    Ok(())
}
