use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SubTask {
    gen_file: String,
    ans_file: String,
    check_file: String,
    score: i8,
}

impl Clone for SubTask {
    fn clone(&self) -> Self {
        SubTask {
            gen_file: self.gen_file.clone(),
            ans_file: self.ans_file.clone(),
            check_file: self.check_file.clone(),
            score: self.score.clone(),
        }
    }
}