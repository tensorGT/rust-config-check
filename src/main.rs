use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn validate_config(content: &str) -> Result<(), Vec<String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut errors: Vec<String> = Vec::new();

    for (idx, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            errors.push(format!("line {}: expected key=value format", idx + 1));
            continue;
        }
        map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
    }

    for req in &["app_name", "port", "env", "max_workers"] {
        if !map.contains_key(*req) {
            errors.push(format!("missing required key: {}", req));
        }
    }

    if let Some(port_str) = map.get("port") {
        match port_str.parse::<u16>() {
            Ok(p) if p >= 1 && p <= 65535 => {}
            _ => errors.push(format!("invalid port '{}': must be 1..65535", port_str)),
        }
    }

    if let Some(env_str) = map.get("env") {
        if !matches!(env_str.as_str(), "dev" | "staging" | "prod") {
            errors.push(format!("invalid env '{}': must be dev, staging, or prod", env_str));
        }
    }

    if let Some(w_str) = map.get("max_workers") {
        match w_str.parse::<u32>() {
            Ok(w) if w >= 1 && w <= 64 => {}
            _ => errors.push(format!("invalid max_workers '{}': must be 1..64", w_str)),
        }
