pub mod cd;
pub mod run;

use std::process::Command;

pub struct Cmd {
    pub inner: Command,
}

impl Cmd {
    pub fn new() -> Self {
        let name = String::new();
        Cmd {
            inner: Command::new(name),
        }
    }
    
    pub fn from(cmd: &str) -> Self {
        let parts: Vec<&str>= cmd.split_whitespace().collect();
        let name = parts[0].to_string();
        let arguments: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        let mut inner = Command::new(name);
        inner.args(arguments);
        Cmd {inner, }
    }

    fn get_program(&self) -> &str {
        self.inner.get_program()
            .to_str().expect("Can't convert program to str")
    }
}
