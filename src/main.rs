use std::fs;
use std::io::{Error, Read};
use std::fs::File;
use rand::RngCore;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json;

type RUSE = Result<(), Error>; //R<US,E>
const OK: Result<(), Error> = Ok(());

const DATA_FILE_PATH: &str = "files/data.json";

#[derive(Debug, Serialize, Deserialize)]
struct KanBansJammed {
    collections: Vec<Collection>,
}

impl KanBansJammed {
    fn new() -> Self {
        Self {
            collections: vec![],
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Collection {
    uuid: String,
    creator: String,
    columns: Vec<String>,
    stickies: Vec<Sticky>
}
#[derive(Debug, Serialize, Deserialize)]
struct Sticky {
    uuid: String,
    creator: String,
    title: String,
    summary: String,
}
fn generate_uuid() -> Uuid {
    let mut rng = rand::thread_rng();
    let mut bytes: [u8; 16] = [0;16];
    rng.fill_bytes(&mut bytes);
    let uuid = Uuid::from_bytes(bytes);
    uuid
}
fn main() -> RUSE {

    println!("trying to load files/data.json");

    let mut data_file: File;
    let open = File::open(DATA_FILE_PATH);
    match open {
        Ok(file) => data_file = file,
        Err(e) => {
            println!("{e}; couldn't open, creating instead");
            let create = File::create_new(DATA_FILE_PATH);
            match create {
                Ok(file) => data_file = file,
                Err(e) => {
                    println!("{e}; couldn't create, making sure directory exists");
                    let create_dir = fs::create_dir("files");
                    match create_dir {
                        Ok(_) => data_file = File::create_new(DATA_FILE_PATH)?,
                        Err(_e) => panic!("{e}")
                    }
                }
            }
        }
    }

    println!("data_file loaded; todo: deserialize contents");
    let mut buff: String = "".to_string();
    let bytes = data_file.read_to_string(&mut buff)?;
    println!("loaded {bytes} bytes from files/data.json");
    let structure: KanBansJammed = serde_json::from_str(&buff.as_str()).unwrap_or({
        println!("error deserializing; starting with empty workspace");
        KanBansJammed::new()
    });

    println!("shutting down; saving workspace to {}", DATA_FILE_PATH);
    let serialized = serde_json::to_string(&structure).expect("should have been able to serialize");
    fs::write(DATA_FILE_PATH, serialized).expect(format!("should have been able to save to {}", DATA_FILE_PATH).as_str());
    println!("saved serialized to {}", DATA_FILE_PATH);
    OK
}

/*
    let first_post = Post::new("arbitrary".to_string(), "random".to_string(), generate_uuid());
    println!("Hello, {}!", first_post.identifier);
    println!("name: {}, creator: {}", first_post.name, first_post.creator);
    let json = serde_json::to_string(&first_post).expect("error; couldn't serialize");
    println!("as json: {}", json);
    let clone_from_serialized: Post = serde_json::from_str(&json).expect("error; couldn't deserialize");
    println!("after deserializing: {clone_from_serialized:?}");
*/