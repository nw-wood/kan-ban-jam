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

    cli::run_cli(&mut board, &config_path);
    
}