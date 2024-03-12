mod subtask;
use subtask::SubTask;
use std::{fs, env};
use std::path::Path;

pub struct Problem {
    online_judge_url: String,
    problem_id: i32,
    problem_name: String,
    subtasks: Vec<SubTask>,
}

impl Problem {
    pub fn new() -> Self {
        Problem {
            online_judge_url: String::new(),
            problem_id: 0,
            problem_name: String::new(),
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
