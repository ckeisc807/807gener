pub mod subtask;
pub mod basics;
pub mod json;
use subtask::SubTask;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub is_empty: bool,
    pub problem_id: i32,
    pub problem_name: String,
    pub path: String,
    pub subtasks: Vec<SubTask>,
}
