extern crate directories;

use std::io::{self, Write};
use std::env;
use std::env::consts::OS;
use std::path::PathBuf;
use gen807::Cmd;
//use directories::BaseDirs;

fn get_home_to_current_path() -> Option<PathBuf> {
    // 獲取家目錄和當前目錄
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
        // 如果是 Linux 或 macOS，返回相對路徑
        Ok(rel_path) if cfg!(any(target_os = "linux", target_os = "macos")) => Some(rel_path.to_path_buf()),
        // 如果是 Windows 或其他作業系統，返回當前目錄的絕對路徑
        _ => Some(current_dir),
    }
}

fn main() {
    println!("In generating mode");
    let mut command = String::new();

    #[allow(unused_assignments)]
    let mut cmd = Cmd::new();

    loop{
        if let Some(path) = get_home_to_current_path() {
            print!("{:?}",path);
        }
        io::stdout().flush()
            .expect("flush error");

        command.clear();
        io::stdin().read_line(&mut command)
            .expect("error command");

        if command.chars().last() == Some('\n') {
            command.pop();
        }
        
        cmd = Cmd::from(&command);
        
        println!("command: {:?}", cmd.inner.get_program());
        println!("arguments: {:?}", cmd.inner.get_args());

        if cmd.inner.get_program() == "exit" {
            break;
        }

        else{
            println!("running");
            let output=cmd.inner.output()
                .expect("error");
            let output_str = String::from_utf8_lossy(&output.stdout);
            print!("{}",output_str);
            println!("finished");
        }
    }
}
