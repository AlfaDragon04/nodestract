use std::fs;
use std::path::Path;
use crate::engine::value::Value;

fn ns_to_serde(val: &Value) -> serde_json::Value {
    match val {
        Value::Null => serde_json::Value::Null,
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::Integer(i) => serde_json::Value::Number((*i).into()),
        Value::Float(f) => {
            if let Some(num) = serde_json::Number::from_f64(*f) {
                serde_json::Value::Number(num)
            } else {
                serde_json::Value::Null
            }
        }
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(ns_to_serde).collect())
        }
        Value::Map(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                obj.insert(k.clone(), ns_to_serde(v));
            }
            serde_json::Value::Object(obj)
        }
    }
}

fn serde_to_ns(val: serde_json::Value) -> Value {
    match val {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::Null
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.into_iter().map(serde_to_ns).collect())
        }
        serde_json::Value::Object(obj) => {
            let mut map = std::collections::HashMap::new();
            for (k, v) in obj {
                map.insert(k, serde_to_ns(v));
            }
            Value::Map(map)
        }
    }
}

pub fn read_file(path: &str) -> Result<Value, String> {
    if !path.ends_with(".json") && !path.ends_with(".txt") {
        return Err("FS Error: Only .json or .txt files allowed.".to_string());
    }
    match fs::read_to_string(path) {
        Ok(content) => {
            if path.ends_with(".json") {
                match serde_json::from_str(&content) {
                    Ok(json_val) => Ok(serde_to_ns(json_val)),
                    Err(e) => {
                        Err(format!("FS Error: Malformed JSON in file '{}'. {}", path, e))
                    }
                }
            } else {
                Ok(Value::String(content))
            }
        }
        Err(e) => Err(format!("FS Error: Could not read file '{}'. {}", path, e)),
    }
}

pub fn write_file(path: &str, content: &Value) -> Result<Value, String> {
    if !path.ends_with(".json") && !path.ends_with(".txt") {
        return Err("FS Error: Only .json or .txt files allowed.".to_string());
    }
    
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }

    let write_res = if path.ends_with(".json") {
        let serde_val = ns_to_serde(content);
        serde_json::to_string_pretty(&serde_val).map_err(|e| e.to_string())
    } else {
        Ok(content.to_string())
    };

    match write_res {
        Ok(content_str) => match fs::write(path, content_str) {
            Ok(_) => Ok(Value::Boolean(true)),
            Err(e) => Err(format!("FS Error: Could not write file '{}'. {}", path, e)),
        },
        Err(e) => Err(format!("FS Error: Serialization failed. {}", e)),
    }
}

pub fn delete_file(path: &str) -> Result<Value, String> {
    if !path.ends_with(".json") && !path.ends_with(".txt") {
        return Err("FS Error: Only .json or .txt files allowed.".to_string());
    }
    match fs::remove_file(path) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(format!("FS Error: Could not delete file '{}'. {}", path, e)),
    }
}