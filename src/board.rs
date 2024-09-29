use serde::{Serialize, Deserialize};
use serde_json;
use std::{fs::{self}, path::{Path, PathBuf}};

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
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

    pub fn open_from_file(path: &Path) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(json) = serde_json::from_str(&contents) {
                println!("loaded board from previous save...");
                return json;
            } else { println!("unable to deserialize json in config file!"); }
        } else { println!("unable to read contents from config file!"); }

        Board::new("kan-ban-board", vec!["new".to_string(), "wip".to_string(), "review".to_string(), "done".to_string()])
    }

    pub fn list_items(&self) {
        println!("printing item list...");
        self.items.iter().for_each(|item| println!("item; name: {}, status: {}, contents: {}", item.name, item.status, item.contents));
    }

    pub fn save(&self, path: &PathBuf) {
        match serde_json::to_string(&self) {
            Ok(json) => {
                match fs::write(&path, json) {
                    Ok(_) => println!("saved board"),
                    Err(e) => println!("couldn't save: {e}, config path: {}", path.display()),
                }
            }
            Err(e) => println!("couldn't serialize to JSON: {e}"),
        }
    }

    pub fn add_item(&mut self, name: &str, contents: &str) {

        if let Some(_)  = self.items.iter().find(|item| item.name == name) { 
            println!("can't add '{name}' because it already exists"); 
        } else { 
            self.items.push(Item::new(name, contents, self.statuses[0].as_str()));
            println!("added item to board; item: {name}, contents: {contents}, status: {}", self.statuses[0].as_str());
        }
    }

    pub fn demote_item(&mut self, name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            if let Some(index) = self.statuses.iter().position(|status| &item.status == status) {
                if index > 0 {

                    item.set_status(&self.statuses[index - 1]);
                    println!("updated status of item; item: {name}, new status: {}", &self.statuses[index - 1]);

                } else { println!("couldn't promote because already lowest possible status"); }
            } else { println!("couldn't find matching status index (shouldn't ever happen)"); }
        } else { println!("couldn't find item '{name}'"); }
    }

    pub fn promote_item(&mut self, name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            if let Some(index) = self.statuses.iter().position(|status| &item.status == status ) {
                if index < self.statuses.len() - 1 {

                    item.set_status(&self.statuses[index + 1]);
                    println!("updated status of item; item: {name}, new status: {}", &self.statuses[index + 1]);

                } else { println!("couldn't promote because already highest possible status"); }
            } else { println!("couldn't find matching status index (shouldn't ever happen)"); }
        } else { println!("couldn't find item '{name}'"); }
    }

    pub fn rename_item(&mut self, name: &str, new_name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {

            item.set_name(new_name);
            println!("set name of {name} to {new_name}");

        } else {
            println!("couldn't find item '{name}'");
        }
    }

    pub fn update_item(&mut self, name: &str, new_contents: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {

            item.set_contents(new_contents);
            println!("set contents of {name} to {new_contents}");

        } else {
            println!("couldn't find item '{name}'");
        }
    }

    pub fn remove_item(&mut self, name: &str) {
        if let Some(index) = self.items.iter().position(|item| item.name == name) {

            self.items.remove(index);
            println!("removed item {name} from the board");

        } else {
            println!("couldn't find item '{name}'");
        }
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
        Self {
            name: name.to_string(),
            contents: contents.to_string(),
            status: status.to_string(),
        }
    }

    fn set_status(&mut self, status: &str)          { self.status = status.to_string(); }
    fn set_name(&mut self, name: &str)              { self.name = name.to_string(); }
    fn set_contents(&mut self, new_contents: &str)  { self.contents = new_contents.to_string(); }
}