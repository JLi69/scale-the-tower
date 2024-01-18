use glfw::Key;
use std::{collections::HashMap, fs::File, io, io::BufRead};

pub type KeyId = i32;
pub type Action = String;
pub struct InputConfig {
    keymap: HashMap<KeyId, Action>,
}

fn parse_lines(lines: &[String]) -> HashMap<KeyId, Action> {
    let mut config = HashMap::new();
    for line in lines {
        //TODO: implement better parsing of input map config file,
        //for now we just assume that it is of the format string1=string2
        //and that there is exactly *1* '=' character in each line
        let split: Vec<String> = line.split('=').map(|s| s.to_string()).collect();
        if split.len() < 2 {
            continue;
        }

        if let Ok(scancode) = split[0].trim().parse::<KeyId>() {
            config.insert(scancode, split[1].trim().to_string());
        }
    }

    config
}

impl InputConfig {
    pub fn get_action(&self, key_id: KeyId) -> Option<Action> {
        return self.keymap.get(&key_id).cloned();
    }

    fn read_config(path: &str) -> Result<Self, String> {
        let reader = io::BufReader::new(File::open(path).map_err(|e| e.to_string())?);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

        Ok(Self {
            keymap: parse_lines(&lines),
        })
    }

    pub fn default() -> Self {
        let default_map = [
            (Key::Space as i32, "Attack".to_string()),
            (Key::Right as i32, "Right".to_string()),
            (Key::Left as i32, "Left".to_string()),
            (Key::Up as i32, "Up".to_string()),
            (Key::Down as i32, "Down".to_string()),
            (Key::Num1 as i32, "Sword".to_string()),
            (Key::Num2 as i32, "Bow".to_string()),
            (Key::Escape as i32, "Escape".to_string()),
        ];

        Self {
            keymap: HashMap::from(default_map),
        }
    }

    pub fn new(config_path: &str) -> Self {
        match Self::read_config(config_path) {
            Ok(config) => config,
            Err(msg) => {
                eprintln!("Failed to open: {config_path}");
                eprintln!("{msg}");
                Self::default()
            }
        }
    }
}
