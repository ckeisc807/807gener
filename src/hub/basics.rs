use super::Hub;
impl Hub {
    pub fn new() -> Self {
        Hub {
            is_empty: true,
            problems: Vec::new(),
            oj_name: String::new(),
            oj_url: String::new(),
            dir_path: String::new(),
        }
    }
}