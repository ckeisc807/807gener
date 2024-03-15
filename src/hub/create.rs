use super::Hub;
use std::{fs, io::Write};
use std::path::PathBuf;

impl Hub {
    pub fn create(&mut self, _oj_name: String, _oj_url: String, _path: String) -> Option<&Hub> {
        let relative_path = PathBuf::from(&_path);
        let hub_path = relative_path.canonicalize().expect("could transfer relative path to absolute path");
        let dir_path = hub_path.to_str().expect("Couldn't convert hub path to str");
        let json_dir_path = format!("{}/config",dir_path);
        let json_path = format!("{}/config/config.json",dir_path);
        
        #[allow(unused_variables)]
        if let Ok(metadata) = fs::metadata(&json_path) {
            eprintln!("Valid to create an exsisted hub");
            return None;
        }

        self.is_empty = false;
        self.problems = Vec::new();
        self.oj_name = _oj_name;
        self.oj_url = _oj_url;
        self.dir_path = dir_path.to_string();

        let json_string = serde_json::to_string(&self).unwrap();
        let _ = fs::create_dir_all(&json_dir_path);
        let mut file = fs::File::create(&json_path).expect("create file error");
        file.write_all(json_string.as_bytes()).unwrap();

        return Some(self);
    }
}