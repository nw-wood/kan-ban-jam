use env_home;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use regex::Regex;

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

    fn list_items(&self) {
        for item in &self.items {
            println!("item; name: {}, status: {}, contents: {}", item.name, item.status, item.contents);
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
        let mut exists: bool = false;
        for item in &mut self.items {
            if item.name == name.to_string() {
                exists = true;
                for (index, status) in &mut self.statuses.iter().enumerate() {
                    if item.status == *status && &item.status != self.statuses.last().unwrap() {
                        item.set_status(&self.statuses[index + 1]);
                        break;
                    } 
                }
            }
        }
        if exists == false { println!("couldn't promote item because it doesn't exist"); }
    }

    fn demote_item(&mut self, name: &str) {
        let mut exists: bool = false;
        for item in &mut self.items {
            if item.name == name.to_string() {
                exists = true;
                for (index, status) in &mut self.statuses.iter().enumerate() {
                    if item.status == *status && item.status != self.statuses[0] {
                        item.set_status(&self.statuses[index - 1]);
                        break;
                    }
                }
            }
        }
        if exists == false { println!("couldn't demote item because it doesn't exist"); }
    }

    fn rename_item(&mut self, name: &str, new_name: &str) {
        let mut exists: bool = false;
        for item in &mut self.items {
            if item.name == name {
                exists = true;
                item.set_name(new_name);
            }
        }
        if exists == false { println!("couldn't rename because the item doesn't exist"); }
    }

    fn update_item_contents(&mut self, name: &str, new_contents: &str) {
        let mut exists: bool = false;
        for item in &mut self.items {
            if item.name == name { item.set_contents(new_contents); }
            exists = true;
        }
        if exists == false { println!("couldn't update contents because the item doesn't exist"); }
    }

    fn remove_item(&mut self, name: &str) {
        let mut exists: bool = false;
        let items_clone = self.items.clone();
        for (index, item) in items_clone.iter().enumerate() {
            if item.name == name {
                self.items.remove(index);
                exists = true;
            }
        }
        if exists == false { println!("couldnt remove the item because it doesn't exist"); }
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    name: String,
    contents: String,
    status: String,
}

impl Item {
    fn new(name: &str, contents: &str, status: &str) -> Self {
        println!("producing an item; {}, {}, {}", name, contents, status);
        Self {
            name: name.to_string(),
            contents: contents.to_string(),
            status: status.to_string(),
        }
    }

    fn set_status(&mut self, status: &str) {
        println!("updating '{}' to status '{}'", self.name, status);
        self.status = status.to_string();
    }

    fn set_name(&mut self, name: &str) {
        println!("updating '{}' to name '{}'", self.name, name);
        self.name = name.to_string();
    }

    fn set_contents(&mut self, new_contents: &str) {
        println!("updating contents of '{}' with new content: {}", self.name, new_contents);
        self.contents = new_contents.to_string();
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

    let mut _buff = String::new();

    loop { 
        _buff = String::new();
        std::io::stdin().read_line(&mut _buff).expect("couldn't read line");

        _buff.pop(); //remove new line

        let split: Vec<&str> = _buff.split_whitespace().collect();

        let re = Regex::new(r#""([^"]+)""#).unwrap();
        let items: Vec<String> = re.captures_iter(&_buff)
            .filter_map(|cap| {
                let item = cap[1].to_string();
                if item.trim().is_empty() {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();

        let name_provided: bool  = items.get(0).is_some();
        let param_provided: bool = items.get(1).is_some();

        match split[0] {
            "exit" => break,
            "help" => println!("list of commands: help, list, exit, new, rename, update, promote, and demote"),
            "new" => {
                if name_provided == true && param_provided == true {
                    board.add_item(items[0].as_str(), items[1].as_str());
                }
            }
            "rename" => {
                if name_provided == true && param_provided == true {
                    board.rename_item(items[0].as_str(), items[1].as_str());
                }
            }
            "update" => {
                if name_provided == true && param_provided == true {
                    board.update_item_contents(items[0].as_str(), items[1].as_str());
                }
            }
            "promote" => {
                if name_provided == true {
                    board.promote_item(items[0].as_str());
                }
            }
            "demote" => {
                if name_provided == true {
                    board.demote_item(items[0].as_str());
                }
            }
            "delete" => {
                if name_provided == true {
                    board.remove_item(items[0].as_str());
                }
            }
            "list" => {
                board.list_items();
            }
            "save" => {
                board.save(&config_path);
            }
            _ => println!("unknown command; type help if you need it"),
        }
    }

    board.save(&config_path);

}