use super::Hub;
use std::{fs, io::Write};
use std::path::{PathBuf, Path};

impl Hub {
    pub fn create(&mut self, _oj_name: String, _oj_url: String, _path: String) -> Option<&Hub> {
/*        let relative_path = PathBuf::from(&_path);
        eprintln!("relative_path {:?}", relative_path);
        let hub_path = relative_path.canonicalize().expect("could transfer relative path to absolute path");
        let dir_path = hub_path.to_str().expect("Couldn't convert hub path to str");
        let json_dir_path = format!("{}/config",dir_path);
        let json_path = format!("{}/config/config.json",dir_path);
        */
        let mut dir_path = PathBuf::new();
        if Path::new(&_path).is_absolute() {
            dir_path.push(&_path);
        }
        else {
            dir_path = std::env::current_dir().expect("Could get current dir")
                .join(&_path);
        }
        let _= fs::create_dir_all(&dir_path);
        let dir_path_str = dir_path.to_str().expect("Error to convert dir path to String")
            .to_string();
        let json_dir_path = format!("{}/config",dir_path_str);
        let json_path = format!("{}/config/config.json",dir_path_str);
 
        #[allow(unused_variables)]
        if let Ok(metadata) = fs::metadata(&json_path) {
            eprintln!("Valid to create an exsisted hub");
            return None;
        }

        self.is_empty = false;
        self.problems = Vec::new();
        self.oj_name = _oj_name;
        self.oj_url = _oj_url;
        self.dir_path = dir_path_str;

        let json_string = serde_json::to_string(&self).unwrap();
        let _ = fs::create_dir_all(&json_dir_path);
        let mut file = fs::File::create(&json_path).expect("create file error");
        file.write_all(json_string.as_bytes()).unwrap();

        return Some(self);
    }
}