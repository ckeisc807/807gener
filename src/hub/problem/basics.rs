use std::{env, fs};
use super::Problem;

impl Problem {
    pub fn new() -> Self {
        Problem {
            is_empty: false,
            problem_id: 0,
            problem_name: String::new(),
            path: String::new(),
            subtasks: Vec::new(),
        }
    }
    
    pub fn create() {
        let current_path = env::current_dir().expect("can't get current path");
        let current_path_str = current_path.to_str().expect("Couldn't convert current path to str");
        let problem_path_str = format!("{}/ans", current_path_str);
        let metadata = fs::metadata(problem_path_str).expect("metadata error");
        println!("{:?}",metadata);
    }
}

impl Clone for Problem {
    fn clone(&self) -> Self {
        Problem {
            is_empty: self.is_empty,
            problem_id: self.problem_id.clone(),
            problem_name: self.problem_name.clone(),
            path: self.path.clone(),
            subtasks: self.subtasks.clone(),
        }
    }
}