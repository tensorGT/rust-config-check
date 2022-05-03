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
