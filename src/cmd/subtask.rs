use crate::hub::problem::subtask::SubTask;
use super::Cmd;

use std::path::Path;

use std::io;

impl Cmd {
    pub fn subtask (&mut self) {
        let args = self.get_args_string();
        let action = args.first().expect("no args for subtask");
        if self.problem.is_none() {
            eprintln!("Not problem selected");
        }

        if action == "create" {
            self.create_subtask();
        }
    }
    pub fn create_subtask(&mut self) {
        let mut subtask_name = String::new();

        println!("Please input subtask name");
        io::stdin().read_line(&mut subtask_name)
            .expect("Failed to input subtask_name");
        if subtask_name.chars().last() == Some('\n') {
            subtask_name.pop();
        }

        println!("Please input subtask score");
        let mut tmp_input = String::new();
        io::stdin().read_line(&mut tmp_input)
            .expect("Failed to input subtask_name");
        let score: i8 = match tmp_input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Failed to input score: {}",e);
                return ;
            }
        };

        let subtask= SubTask {
            gen_file: format!("{}.cpp",subtask_name),
            check_file: format!("{}.cpp", subtask_name),
            score: score,
        };
        let problem = self.problem.as_mut().expect("no problem selected");
        problem.subtasks.push(subtask);

        let problem_path = Path::new(&problem.path);
        problem.to_json(&problem_path);

        let hub = self.hub.as_mut().expect("no hub selected");
        let hub_path = Path::new(&hub.dir_path);
        hub.to_json(&hub_path);
    }
}