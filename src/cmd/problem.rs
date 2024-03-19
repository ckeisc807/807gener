use crate::Hub;

use super::Problem;
use super::Cmd;
use std::path::PathBuf;
use std::io;


impl Cmd {
    pub fn problem(&mut self) {
        let args = self.get_args_string();
        let action = args.first().expect("no args for hub");
        if action == "create" {
            self.create_problem();
        }
        else if action == "select" {
            self.select_problem();
        }
    }

    pub fn create_problem(&mut self) {
        let mut tmp_input = String::new();

        println!("Please input problem id");
        io::stdin().read_line(&mut tmp_input).expect("Failed to read line");
        let problem_id: i32 = match tmp_input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Failed to input i32: {}", e);
                return ;
            }
        };

        println!("Please input problem name");
        let mut problem_name = String::new();
        io::stdin().read_line(&mut problem_name).expect("Failed to read line");
        if problem_name.chars().last() == Some('\n') {
            problem_name.pop();
        }

        let hub: &mut Hub = &mut self.hub.as_mut().expect("hub not found");
        let mut problem_path = PathBuf::from(&hub.dir_path);

        problem_path.push(problem_id.to_string());

        let problem = Problem {
            is_empty: false,
            problem_id: problem_id,
            problem_name: problem_name,
            path: problem_path.to_string_lossy().to_string(),
            subtasks: Vec::new(),
        };

        problem.to_json(&problem_path);
        self.problem = Some(problem.clone());
        
        hub.problems.push(problem);
        hub.to_json(&problem_path.parent().expect("Couldn't find Hub directory"));
    }

    pub fn select_problem(&mut self) {
        
    }
}