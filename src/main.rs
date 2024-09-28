use env_home;
use std::{fs::{self}, path::{Path, PathBuf}, process};
use serde::{Serialize, Deserialize};
use serde_json;
use regex::Regex;


#[derive(Deserialize, Serialize, Debug)]
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

    fn open_from_file(path: &Path) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(json) = serde_json::from_str(&contents) {
                return json;
            } else { println!("unable to deserialize json in config file"); }
        } else { println!("unable to read contents from config file"); }

        Board::new("kan-ban-board", vec!["new".to_string(), "wip".to_string(), "review".to_string(), "done".to_string()])
    }

    fn list_items(&self) {
        self.items.iter().for_each(|item| println!("item; name: {}, status: {}, contents: {}", item.name, item.status, item.contents));
    }

    fn save(&self, path: &PathBuf) {
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

    fn add_item(&mut self, name: &str, contents: &str) {

        if let Some(_)  = self.items.iter().find(|item| item.name == name) { 
            println!("can't add '{name}' because it already exists"); 
        } else { 
            self.items.push(Item::new(name, contents, self.statuses[0].as_str()));
        }
    }

    fn demote_item(&mut self, name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            if let Some(index) = self.statuses.iter().position(|status| &item.status == status) {
                if index > 0 {
                    item.set_status(&self.statuses[index - 1]);
                } else { println!("couldn't promote because already lowest possible status"); }
            } else { println!("couldn't find matching status index (shouldn't ever happen)"); }
        } else { println!("couldn't find item '{name}'"); }
    }

    fn promote_item(&mut self, name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            if let Some(index) = self.statuses.iter().position(|status| &item.status == status ) {
                if index < self.statuses.len() - 1 {
                    item.set_status(&self.statuses[index + 1]);
                } else { println!("couldn't promote because already highest possible status"); }
            } else { println!("couldn't find matching status index (shouldn't ever happen)"); }
        } else { println!("couldn't find item '{name}'"); }
    }

    fn rename_item(&mut self, name: &str, new_name: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            item.set_name(new_name);
        } else {
            println!("couldn't find item '{name}'");
        }
    }

    fn update_item(&mut self, name: &str, new_contents: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            item.set_contents(new_contents);
        } else {
            println!("couldn't find item '{name}'");
        }
    }

    fn remove_item(&mut self, name: &str) {
        if let Some(index) = self.items.iter().position(|item| item.name == name) {
            self.items.remove(index);
        } else {
            println!("couldn't find item '{name}'");
        }
    }

    fn item_exists(&self, name: &str) -> bool {
        if let Some(_) = self.items.iter().find(|item| item.name == name) {
            true
        } else {
            false
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

const BOARD_CONFIG: &str = "/.config/kan-ban-jam/kanban_config.json";

fn get_config_path(str_path: &str) -> PathBuf {
    match env_home::env_home_dir() {
        Some(directory) => Path::new(&format!("{}/{}", directory.display(), str_path)).to_path_buf(),
        None => {
            match std::env::current_dir() {
                Ok(directory) => Path::new(&format!("{}", directory.display())).to_path_buf(),
                Err(e) => panic!("couldnt resolve a working directory; error: {e}"),
            }
        },
    }
}

fn is_valid_simple_command(board: &Board, inputs: &Vec<&str>) -> bool {
    if let Some(&second_input) = inputs.get(1) {
        if board.item_exists(second_input) == true {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn is_valid_quoted_single_input(inputs: &Vec<String>) -> bool {

    if inputs.get(0).is_some() { true } else { false }
}

fn is_valid_quoted_double_input(inputs: &Vec<String>) -> bool {

    if inputs.get(0).is_some() && inputs.get(1).is_some() { true }
    else { false }
}

fn main() {

    let config_path = get_config_path(BOARD_CONFIG);

    let mut board = Board::open_from_file(&config_path);

    println!("board loaded; printing a quick list of items...");

    board.list_items();

    let mut _buff = String::new();

    loop { 
        _buff = String::new();
        std::io::stdin().read_line(&mut _buff).expect("couldn't read line");

        _buff.pop(); //remove new line

        let simple_inputs: Vec<&str> = _buff.split_whitespace().collect();

        let re = Regex::new(r#""([^"]+)""#).unwrap();
        let quoted_inputs: Vec<String> = re.captures_iter(&_buff)
            .filter_map(|cap| {
                let item = cap[1].to_string();
                if item.trim().is_empty() {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();

        let inputs: Vec<&str> = {
            if is_valid_simple_command(&board, &simple_inputs) {
                simple_inputs.clone()
            } else if is_valid_quoted_single_input(&quoted_inputs) {
                if is_valid_quoted_double_input(&quoted_inputs) {
                    vec![simple_inputs[0], quoted_inputs[0].as_str(), quoted_inputs[1].as_str()]
                } else if is_valid_quoted_single_input(&quoted_inputs) {
                    vec![simple_inputs[0], quoted_inputs[0].as_str()]
                } else {
                    vec!["format"]
                }
            } else {
                vec!["format"]
            }
        };

        match inputs.as_slice() {
            [command, name, param] => {
                match *command {
                    "add" =>    board.add_item(name, param),
                    "rename" => board.rename_item(name, param),
                    "update" => board.update_item(name, param),
                    _ => println!("unknown command, or provided too many params"),
                }
            },
            [command, name] => {
                match *command {

                    "promote" => board.promote_item(name),
                    "demote" => board.demote_item(name),
                    "remove" => board.remove_item(name),
                    _ => println!("unknown command, or provided too many params"),
                }
            },
            [command] => {
                match *command {
                    "list" => board.list_items(),
                    "help" => println!("commands are list, help, format, rename, update, remove, promote, demote, add, and exit"),
                    "format" => println!("commands must be provided as 'command \"name wth spaces\" \"second input with spaces\"'; or simple commands as 'command name_without_spaces second input with or without spaces or quotes"),
                    // => "unknown command",
                    _ => println!("unknown command"),
                }
            },
            _ => println!("what in the fuck"),
        }
        board.save(&config_path);
    }

}

//notes about asyncronous operations
//use async when you need I/O or long-running tasks
//don't block the thread in async functions
//keep async functions small

//use await for async tasks, but limit its usage

    /*
        async fn fetch_data() {
            let data = async_get_data().await;  // Good
            process_data(data);  // Processing happens after awaiting
        }
    */

//handle async errors explcitly - when writing async functions return results<t, e>'s as good practice, and handle errors from them

    /*
        async fn fetch_data() -> Result<String, SomeError> {
            let response = http_call().await?;
            Ok(response)
        }
    */

//be careful with data with a shared state, like data contained in an arc mutex, or a tokio mutex
    /*
        let shared_data = Arc::new(Mutex::new(vec![])); <--- careful handling this, do futures hold locks? IDK, maybe, but during execution via awaiting them they certainly would
    */

//avoid using async in constructors (like associated new functions)

    /*
        impl MyStruct {
            fn new() -> Self { /* synchronous */ }
            async fn initialize(&mut self) { /* async setup */ }
        }
    */

//use tokio::spawn for parallel execution

    /*
        let handle1 = tokio::spawn(async_task1()); <-------- future for task is created and bound to handle1
        let handle2 = tokio::spawn(async_task2());

        let result1 = handle1.await?;  <-------------------- the future is executed now for handle1, but just as well if handle2.await?; was called then out of order exec would have been fine
        let result2 = handle2.await?;
    */

//understand that async functions return futures *

    /*
        async fn example() -> u32 {
            42
        }

        let future = example();  // Returns a Future, but doesn't run
        let result = future.await;  // Now the future runs

        #[derive(Serialize, Deserialize, Debug)]
        struct Board {
            name: String,
            items: Vec<Item>,
            statuses: Vec<String>,
        }
    */

//use timeouts instead of sleeping threads and stuff - tokio has a way of executing a future after a duration of time without breaking things

    //let result = tokio::time::timeout(Duration::from_secs(5), async_task()).await;