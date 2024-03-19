use super::Problem;
use std::path::Path;
use std::io::{Read, Write};
use std::fs;

impl Problem {
    pub fn from_json(problem_path: &Path) -> Option<Self> {
        let mut json_file_path = problem_path.to_path_buf();
        json_file_path.push("config.json");
        
        if let Err(_) = fs::metadata(&json_file_path) {
            return None;
        }

        let mut json_file = fs::File::open(&json_file_path)
            .map_err(|err| format!("Failed to open file: {}",err))
            .expect("Failed to open file");
        
        let mut json_str = String::new();
        json_file.read_to_string(&mut json_str)
            .map_err(|err| format!("Failed to read file: {}",err))
            .expect("Failed to read file");

        let problem = serde_json::from_str(&json_str)
            .map_err(|err| format!("Failed to parse JSON: {}",err))
            .expect("Failed to parse JSON");

        Some(problem)
    }

    pub fn to_json(&self, problem_path: &Path) -> Option<&Self> {
        let mut json_file_path = problem_path.to_path_buf();
        json_file_path.push("config.json");
        
        if !problem_path.exists(){
            let _ = fs::create_dir_all(&problem_path).expect("create dir error");
        }
        
        let json_string = serde_json::to_string(&self).unwrap();
        let mut file = fs::File::create(&json_file_path).expect("create file error");
        file.write_all(json_string.as_bytes()).unwrap();

        return Some(self);
    }
}