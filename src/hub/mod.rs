pub mod problem;
pub mod basics;
pub mod create;
pub mod json;

use problem::Problem;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hub {
    pub is_empty: bool,
    pub problems: Vec<Problem>,
    pub oj_name: String,
    pub oj_url: String,
    pub dir_path: String,
}

impl Clone for Hub {
    fn clone(&self) -> Self {
        Hub {
            is_empty: self.is_empty,
            problems: self.problems.clone(),
            oj_name: self.oj_name.clone(),
            oj_url: self.oj_url.clone(),
            dir_path: self.dir_path.clone(),
        }
    }
}
