use serde::{Serialize, Deserialize};
use serde_json;
use core::fmt;
use std::{fs::{self}, path::{Path, PathBuf}};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    board_name: String,
    items: Vec<Item>,
    statuses: Vec<Status>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Status {
    New,
    WorkInProgress,
    Review,
    Done,
}

impl Status {
    fn next(&self) -> Option<Status> {
        match self {
            Status::New => Some(Status::WorkInProgress),
            Status::WorkInProgress => Some(Status::Review),
            Status::Review => Some(Status::Done),
            Status::Done => { println!("unable to demote item"); None },
        }
    }
    fn previous(&self) -> Option<Status> {
        match self {
            Status::New => { println!("unable to promote item"); None },
            Status::WorkInProgress => Some(Status::New),
            Status::Review => Some(Status::WorkInProgress),
            Status::Done => Some(Status::Review),
        }
    }
    fn all() -> Vec<Status> {
        vec![
            Status::New,
            Status::WorkInProgress,
            Status::Review,
            Status::Done,
        ]
    }
}

impl fmt::Display for Status {
    //TODO: &mut fmt::Formatter<'_> - I don't understand the provided '_ type
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::New => write!(f, "New"),
            Status::WorkInProgress => write!(f, "Work in Progress"),
            Status::Review => write!(f, "Review"),
            Status::Done => write!(f, "Done"),
        }
    }
}

impl Board {

    pub fn new(name: &str) -> Self {
        Self {
            board_name: name.to_string(),
            items: vec![],
            statuses: Status::all(),
        }
    }

    fn get_name(&self) -> &str {
        &self.board_name
    }

    pub fn serialized(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn open_from_file(path: &Path) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(deserialized_board) = serde_json::from_str::<Board>(&contents) {

                println!("{}: loaded board from previous save...", deserialized_board.get_name());
                return deserialized_board;

            } else { println!("unable to deserialize json in config file!"); }
        } else { println!("unable to read contents from config file!"); }

        Board::new("default-board")
    }

    pub fn list_items(&self) {
        println!("printing item lists...");
        self.items.iter().for_each(|item| println!("item; name: '{}', status: '{}', contents: '{}'", item.name, item.status, item.contents));
    }

    pub fn save(&self, path: &PathBuf) {
        match serde_json::to_string(&self) {
            Ok(json) => {
                match fs::write(&path, json) {
                    Ok(_) => println!("saved board!"),
                    Err(e) => println!("couldn't save; '{e}', config path: '{}'", path.display()),
                }
            }
            Err(e) => println!("couldn't serialize to JSON: '{e}'"),
        }
    }

    pub fn add_item(&mut self, name: &str, contents: &str) {

        if let Some(_)  = self.items.iter().find(|item| item.name == name) { 
            println!("can't add '{name}' because it already exists"); 
        } else {
            self.items.push(Item::new(name, contents));
            println!("added item to board; item: '{name}', contents: '{contents}', status: '{}'", &self.statuses[0]);
        }
    }

    pub fn demote_item(&mut self, name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            item.demote();
        } else { println!("couldn't find item '{name}'"); }
    }

    pub fn promote_item(&mut self, name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            item.promote();
        } else { println!("couldn't find item '{name}'"); }
    }

    pub fn rename_item(&mut self, name: &str, new_name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {

            item.set_name(new_name);
            println!("set name of '{name}' to '{new_name}'");

        } else {
            println!("couldn't find item '{name}'");
        }
    }

    pub fn update_item(&mut self, name: &str, new_contents: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {

            item.set_contents(new_contents);
            println!("set contents of '{name}' to '{new_contents}'");

        } else {
            println!("couldn't find item '{name}'");
        }
    }

    pub fn remove_item(&mut self, name: &str) {
        if let Some(index) = self.items.iter().position(|item| item.name == name) {

            self.items.remove(index);
            println!("removed item '{name}' from the board");

        } else {
            println!("couldn't find item '{name}'");
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    name: String,
    contents: String,
    status: Status,
}

impl Item {

    fn new(name: &str, contents: &str) -> Self {
        Self {
            name: name.to_string(),
            contents: contents.to_string(),
            status: Status::New,
        }
    }

    fn promote(&mut self) {
        if let Some(status) = self.status.next() {
            self.status = status;
        }
    }

    fn demote(&mut self) {
        if let Some(status) = self.status.previous() {
            self.status = status;
        }
    }

    fn set_name(&mut self, name: &str)              { self.name = name.to_string(); }
    
    fn set_contents(&mut self, new_contents: &str)  { self.contents = new_contents.to_string(); }
}