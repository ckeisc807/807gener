use super::Cmd;
use std::{io, env};
use crate::Hub;


impl Cmd {
    pub fn hub(&mut self) {
        let args = self.get_args_string();
        let action = args.first().expect("no args for hub");
        if action == "create" {
            self.create();
        }
    }

    pub fn create(&mut self) {
        let mut oj_name = String::new();
        let mut oj_url = String::new();
        let mut hub_path = String::new();
        
        println!("Please input oj_name");
        io::stdin().read_line(&mut oj_name)
            .expect("Error oj_name input");

        println!("Please input oj_url");
        io::stdin().read_line(&mut oj_url)
            .expect("Error oj_url input");

        println!("Please input hub_path");
        io::stdin().read_line(&mut hub_path)
            .expect("Error hub_path input");
        if hub_path.chars().last() == Some('\n') {
            hub_path.pop();
        }
        if hub_path.starts_with("~") {
            #[allow(deprecated)]
            match env::home_dir() {
                Some(path) => hub_path = hub_path.replace("~", &path.to_string_lossy()),
                None => {
                eprintln!("Failed to get home directory");
                    return ;
                }
            };
        }
        
        if self.hub.is_none() {
            self.hub = Some(Hub::new());
        }

        match &mut self.hub {
            Some(ref mut value) => {
                println!("value  exsist");
                value.create(oj_name,oj_url,hub_path)
                    .expect("create failed");
            }
            None => {
                eprintln!("Hub hasn't created correctly");
            }
        }
    }
}