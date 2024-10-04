mod board;
mod server;
use env_home;
use std::path::{Path, PathBuf};
use board::Board;
use server::server_main;

use std::sync::Arc;
use std::sync::Mutex;

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

fn main() {

    let config_path = get_config_path(BOARD_CONFIG);

    let board = Board::open_from_file(&config_path);

    server_main(Arc::new(Mutex::new(board)), &config_path);

}

//TODO: spend time with async as a keyword - make more simple examples work
//TODO: spend more time with Paths and PathBuf and learn more of their methods as they are fairly common
//TODO: although not explicitly written here, there is an await after this locking func I believe, and I should spend more time understanding await
//TODO: I believe this is a mspc impl that gets dropped immediately after use, but will have to read into it
//TODO: Spend more time setting up and tinkering with less or even more complicated warp filter combinations
//TODO: Reread the rust book documentation on some of the available smart pointers like Rc and RefCells and Arc
//TODO: Understand how atomics works better because right now it's an 'lol thread safe variable'
//TODO: I'm not entirely sure how this ws return implementation fires back the ws to the map - I need to learn what 'upgrading' is
//TODO: test and make up examples of Trait bound implementations because this is excessively difficult to interpret
    //  the bind with graceful shutdown method has so many of them I really couldn't say what every single one implied all together
    //  some of the trait bounds here are fairly common and I should learn better usage anyways (clone, send, sync, 'static)
//TODO: setup small arbitrary examples of mspcs, oneshots, broadcast channels and so on to experiment and learn
//TODO: look into message implementaions for warp filters (it's various methods and usage is kind of a mystery)
//TODO: understand sinks and streams in this context, the spit function here seperates them, and I believe they basically refer to the transmit and recv
    //  end of message passing functionality
//TODO: try to think like this more often... my thoughts were the message senders into a vector, but making them into an object
        //  and putting those into a vector makes a lot more sense, and makes them more manageable since methods could be implemented on
        //  the clients structure now