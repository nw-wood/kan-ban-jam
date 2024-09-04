use std::fs;
use std::io::{Error};
use std::fs::File;

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

/*
use rand::RngCore;
use uuid::Uuid;

use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    name: String,
    creator: String,
    identifier: String
}

impl Post {
    fn new(name: String, creator: String, id: Uuid) -> Self {
        Self {
            name,
            creator,
            identifier: id.to_string()
        }
    }
}

fn generate_uuid() -> Uuid {
    let mut rng = rand::thread_rng();
    let mut bytes: [u8; 16] = [0;16];
    rng.fill_bytes(&mut bytes);
    let uuid = uuid::Uuid::from_bytes(bytes);
    uuid
}

    let first_post = Post::new("arbitrary".to_string(), "random".to_string(), generate_uuid());
    println!("Hello, {}!", first_post.identifier);
    println!("name: {}, creator: {}", first_post.name, first_post.creator);
    let json = serde_json::to_string(&first_post).expect("error; couldn't serialize");
    println!("as json: {}", json);
    let clone_from_serialized: Post = serde_json::from_str(&json).expect("error; couldn't deserialize");
    println!("after deserializing: {clone_from_serialized:?}");
*/