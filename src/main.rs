pub mod hub;
use hub::Hub;
pub mod cmd;
use cmd::Cmd;

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

    loop{
        if cmd.hub.is_none() {
            let path = get_home_to_current_path().expect("err");
            print!("807gener:{}>",path);
        }
        else {
            if let Some(hub) = &cmd.hub {
                print!("selected hub:{}>", hub.oj_name);
            }
        }
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
        cmd.cmd(&command);
        
        if cmd.inner.get_program() == "exit" {
            break;
        }
        else {
            cmd.run();
        }
    }
}

