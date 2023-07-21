use std::error::Error;
use std::fs;
use std::path;
use std::path::Path;

pub fn read(filename: &str) -> Result<String, Box<dyn Error>> {

    match Path::new(filename).try_exists() {
        Ok(val) => {
            if val {
                return fs::read_to_string(filename);
            } else {
                return None;
            }
        },
        Err(val) => {
            return None;
        }
    }
}