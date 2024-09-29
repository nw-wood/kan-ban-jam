mod board;
mod cli;

use env_home;
use std::path::{Path, PathBuf};

use board::Board;

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

    let mut board = Board::open_from_file(&config_path);

    println!("cli or server? (type cli or server)");

    let mut _buff: String = String::new();

    loop {

        _buff = String::new();
        let _ = std::io::stdin().read_line(&mut _buff);
        _buff.pop();

        if _buff == "cli".to_string() {

            cli::run_cli(&mut board, &config_path);

        } else if _buff == "server".to_string() {

            println!("todo: server impl");
            break;

        }
    }

}