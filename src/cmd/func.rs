use std::process::Command;
use super::Cmd;

impl Cmd {
    pub fn new() -> Self {
        let name = String::new();
        Cmd {
            inner: Command::new(name),
            hub: None,
            problem: None,
            subtask: None,
        }
    }
    
    pub fn from(cmd: &str) -> Self {
        let parts: Vec<&str>= cmd.split_whitespace().collect();
        let name = parts[0].to_string();
        let arguments: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        let mut _inner = Command::new(name);
        _inner.args(arguments);
        Cmd {
            inner: _inner,
            hub: None,
            problem: None,
            subtask: None,
        }
    }

    pub fn get_program(&self) -> &str {
        self.inner.get_program()
            .to_str().expect("Can't convert program to str")
    }

    pub fn get_args_string(&self) -> Vec<String> {
        self.inner.get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect()
    }
}