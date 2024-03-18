pub mod cd;
pub mod run;
pub mod hub;
pub mod func;

use crate::Hub;
use crate::hub::problem::Problem;
use crate::hub::problem::subtask::SubTask;

use std::process::Command;

pub struct Cmd {
    pub inner: Command,
    pub hub: Option<Hub>,
    pub problem: Option<Problem>,
    pub subtask: Option<SubTask>,
}

