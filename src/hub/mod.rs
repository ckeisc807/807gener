pub mod problem;
pub mod new;
pub mod create;
use problem::Problem;

use serde::Serialize;

#[derive(Serialize)]
pub struct Hub {
    is_empty: bool,
    problems: Vec<Problem>,
    oj_name: String,
    oj_url: String,
    dir_path: String,
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

pub use Hub as OtherHub;