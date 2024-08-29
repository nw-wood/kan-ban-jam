//use serde_json::{Deserializer, Serializer};

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

fn main() {

    let first_post = Post::new("arbitrary".to_string(), "random".to_string(), generate_uuid());
    println!("Hello, {}!", first_post.identifier);
    println!("name: {}, creator: {}", first_post.name, first_post.creator);

    let json = serde_json::to_string(&first_post).expect("error; couldn't serialize");
    println!("as json: {}", json);
    let clone_from_serialized: Post = serde_json::from_str(&json).expect("error; couldn't deserialize");
    println!("after deserializing: {clone_from_serialized:?}");
}
