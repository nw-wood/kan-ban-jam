mod board;
mod cli;
mod server;

use env_home;
use std::path::{Path, PathBuf};

use board::Board;
use cli::cli_main;
use server::server_main;

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

    //server_main(&mut board, &config_path); //rather do this faster for development

    println!("cli or server? (type c or s)");

    let mut _buff: String = String::new();

    _buff = String::new();
    let _ = std::io::stdin().read_line(&mut _buff);
    _buff.pop();

    if _buff == "c".to_string() {

        cli_main(&mut board, &config_path);

    } else if _buff == "s".to_string() {

        server_main(&mut board, &config_path);

    }

}