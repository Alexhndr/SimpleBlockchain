use std::io::prelude::Read;
use std::fs::File;
use serde::Deserialize;
use serde_json;
use serde_json::Result;

#[derive(Deserialize)]
pub struct Settings {
    socket_address: String,
}

const SETTINGS_FILENAME: &str = "settings.json";

impl Settings {
    pub fn new() -> Settings {
        Settings::load_from_file()
    }
    
    pub fn socket_address(&self) -> &str {
        self.socket_address.as_str()
    }
    
    pub fn load_from_file() -> Settings {
        let file = File::open(SETTINGS_FILENAME);
        
        match file {
            Ok(_) => {
            },
            Err(error) => panic!("Error opening file \"settings.json\": {}", error),
        }
        
        let mut contents = String::new();
        
        match file.unwrap().read_to_string(&mut contents) {
            Ok(_) => {
            },
            Err(error) => panic!("Error reading file \"settings.json\": {}", error),
        }
        
        let settings: Result<Settings> = serde_json::from_str(contents.as_str());
        
        match settings {
            Ok(settings) => settings,
            Err(error) => panic!("Error reading file \"settings.json\": {}", error),
        }
    }
}
