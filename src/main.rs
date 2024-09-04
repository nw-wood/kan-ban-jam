use std::fs;
use std::io::{Error};

use std::fs::File;

struct Collection {
    columns: Vec<String>
}

type RUSE = Result<(), Error>; //R<US,E>
const OK: Result<(), Error> = Ok(());

fn main() -> RUSE {

    println!("trying to load files/data.json");

    let mut data_file: File;
    let open = File::open("files/data.json");
    match open {
        Ok(file) => data_file = file,
        Err(e) => {
            println!("{e}; couldn't open, creating instead");
            let create = File::create_new("files/data.json");
            match create {
                Ok(file) => data_file = file,
                Err(e) => {
                    println!("{e}; couldn't create, making sure directory exists");
                    let create_dir = fs::create_dir("files");
                    match create_dir {
                        Ok(_) => data_file = File::create_new("files/data.json")?,
                        Err(_e) => panic!("{e}")
                    }
                }
            }
        }
    }
    println!("data_file loaded; todo: deserialize contents");


    OK

}