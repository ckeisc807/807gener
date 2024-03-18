use super::Hub;

use std::path::Path;
use std::fs;
use std::io::Read;

impl Hub {
    pub fn from_json (hub_path: &Path) -> Option<Self> {
        let mut json_file_path = hub_path.to_path_buf();
        json_file_path.push("config");
        json_file_path.push("config.json");

        if let Err(_) = fs::metadata(&json_file_path) {
            return None;
        }

        let mut json_file = fs::File::open(&json_file_path)
            .map_err(|err| format!("Failed to open file: {}",err))
            .expect("Failed to open file");

        let mut json_str = String::new();
        json_file.read_to_string(&mut json_str)
            .map_err(|err| format!("Failed to read file {}",err))
            .expect("Failed to read file");
        
        let hub = serde_json::from_str(&json_str)
            .map_err(|err| format!("Failed to parse JSON: {}",err))
            .expect("Failed to parse JSON");

        Some(hub)
    }
}