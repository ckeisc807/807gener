use std::process::Command;
use std::env;

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

        Cmd {inner}
    }

    pub fn run(&mut self) {
        if self.inner.get_program() == "cd" {
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
        else {
            println!("running");
            let output= self.inner.output()
                .expect("error");
            let output_str = String::from_utf8_lossy(&output.stdout);
            print!("{}",output_str);
            println!("finished");
        }
    }
}
