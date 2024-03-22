pub mod basics;

use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SubTask {
    pub gen_file: String,
    pub check_file: String,
    pub score: i8,
}

