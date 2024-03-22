use std::env;
use super::Cmd;

impl Cmd{
    pub fn cd (&mut self) {
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
                env::set_current_dir(&home_dir)
                    .expect("Couldn't change into home directory");
            }
            return ;
        }
        let new_dir = args.last().expect("err")
            .to_str().expect("error to convert to string");
        let mut new_dir_string = String::from(new_dir);
        if new_dir.starts_with("~"){
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
