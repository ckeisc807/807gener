mod hub;
use hub::Hub;

use std::process::Command;
use std::io::{self, Write};
use std::env;

fn get_home_to_current_path() -> Option<String> {
    #[allow(deprecated)]
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to get home directory");
            return None;
        }
    };
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return None;
        }
    };

    // 將當前目錄轉換為相對於家目錄的路徑
    let relative_path = current_dir.strip_prefix(home_dir);
    match relative_path {
        // OS == Linux || MacOS
        Ok(rel_path) if cfg!(any(target_os = "linux", target_os = "macos")) => Some(format!("~/{}",rel_path.to_str().expect("err").to_string())),
        // OS == Windows
        _ => Some(current_dir.to_str().expect("err").to_string())
    }
}

fn main() {
    println!("In generating mode");
    let mut command = String::new();
    #[allow(unused_assignments)]
    let mut cmd = Cmd::new();
    let mut hub = Hub::new();
    hub.create("iscoj".to_string(),"https://iscoj.fg.tp.edu.tw".to_string(),"./".to_string())
        .expect("error to create");

    loop{
        let path = get_home_to_current_path().expect("err");
        print!("807gener:{}>",path);
        io::stdout().flush()
            .expect("flush error");
        
        //input
        command.clear();
        io::stdin().read_line(&mut command)
            .expect("error command");
        if command.chars().last() == Some('\n') {
            command.pop();
        }
        
        //build up command
        cmd = Cmd::from(&command);
        
        if cmd.inner.get_program() == "exit" {
            break;
        }
        else {
            cmd.run();
        }
    }
}

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

    pub fn run(&mut self) {
        let program = self.get_program();
        if program == "cd" {
            self.cd()
        }
        else if program == "problem" {

        }
        else {
            println!("running");
            let output= self.inner.output()
                .expect("error");
            let output_str = String::from_utf8_lossy(&output.stdout);
            print!("{}",output_str);
            println!("finished");
        }
    }

    fn cd (&mut self) {
        let args = self.inner.get_args();
        let sz=args.len();
        if sz > 1 {
            println!("cd: too many arguments");
            return ;
        }
        else if sz == 0 {
            if cfg!(any(target_os = "linux", target_os = "macos")) {
                #[allow(deprecated)]
                let home_dir = env::home_dir().expect("Couldn't reach home dir");
                env::set_current_dir(home_dir)
                    .expect("Couldn't change into home directory");
            }
            return ;
        }
        let new_dir = args.last().expect("err")
            .to_str().expect("error to convert to string");
        let mut new_dir_string = String::from(new_dir);
        if new_dir.starts_with("~/"){
            #[allow(deprecated)]
            let home_dir = env::home_dir().expect("Couldn't reach home dir");
            let home_dir_str = home_dir.to_str().expect("Covert home_dir to string failed");
            new_dir_string = new_dir_string.replace("~", &home_dir_str);
            println!("replace to {}",&new_dir_string);
        }
        match env::set_current_dir(&new_dir_string) {
            Ok(success) => println!("{:?} change directory to {:?}", success,  new_dir_string),
            Err(e) => eprintln!("failed to change directory {:?}", e),
        }
    }
}
