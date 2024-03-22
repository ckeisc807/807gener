use super::SubTask;

impl SubTask {
    pub fn new() -> Self {
        SubTask {
            gen_file: String::new(),
            check_file: String::new(),
            score: 0,
        }
    }
}

impl Clone for SubTask {
    fn clone(&self) -> Self {
        SubTask {
            gen_file: self.gen_file.clone(),
            check_file: self.check_file.clone(),
            score: self.score.clone(),
        }
    }
}