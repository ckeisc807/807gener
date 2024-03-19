use super::Cmd;

impl Cmd {
    pub fn run(&mut self) {
        let program = self.get_program();
        if program == "cd" {
            self.cd()
        }
        else if program == "hub" {
            self.hub()
        }
        else if program == "problem" {
            self.problem()
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