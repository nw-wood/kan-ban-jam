use env_home;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Board {
    name: String,
    items: Vec<Item>,
    statuses: Vec<String>,
}
impl Board {

    fn new(name: &str, statuses: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            items: vec![],
            statuses,
        }
    }

    fn open_from_file(path: &String) -> Self {

        let file = fs::read_to_string(path);

        if let Ok(contents) = file {

            println!("read file; deserializing");
            let value = serde_json::from_str(contents.as_str());

            if let Ok(json) = value {
                println!("loaded config");
                json
            } else {
                println!("couldnt deserialize; new default");
                Board::new("kan-ban-board", vec!["new".to_string(), "wip".to_string(), "review".to_string(), "done".to_string()])
            }
        } else {
            println!("couldn't read file; new default");
            Board::new("kan-ban-board", vec!["new".to_string(), "wip".to_string(), "review".to_string(), "done".to_string()])
        }
    }

    fn save(&self, path: &String) {
        match serde_json::to_string(&self) {
            Ok(json) => {
                match fs::write(&path, json) {
                    Ok(_) => println!("saved board"),
                    Err(e) => println!("couldn't save: {e}, config path: {path}"),
                }
            }
            Err(e) => println!("couldn't serialize to JSON: {e}"),
        }
    }

    fn add_item(&mut self, name: &str, contents: &str) {
        let mut exists: bool = false;
        for item in &self.items {
            if item.name == name { exists = true }
        }
        if exists {
            println!("can't add {name} because {name} already exists in the board");
        } else { 
            let item = Item::new(name, contents, self.statuses[0].as_str());
            self.items.push(item);
        }
    }

    fn promote_item(&mut self, name: &str) {
        for item in &self.items {
            if item.name == name.to_string() {
                for status in &self.statuses {
                    if item.status == status {

                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    contents: String,
    status: String,
}

impl Item {
    fn new(name: &str, contents: &str, status: &str) -> Self {
        Self {
            name: name.to_string(),
            contents: contents.to_string(),
            status: status.to_string(),
        }
    }
}

const BOARD_CONFIG: &str = "/.config/kan-ban-jam/kanban_config.json";

fn get_config_path() -> String {
    match env_home::env_home_dir() {
        Some(dir) => dir.into_os_string().into_string().unwrap(),
        None => panic!("environment variable for user's home directory doesn't exist!"),
    }
}

fn main() {

    let config_path: String = get_config_path() + BOARD_CONFIG;
    let mut board = Board::open_from_file(&config_path);
    board.add_item("dick", "beets");
    println!("so what's in that board: {board:?}");
    board.save(&config_path);

}