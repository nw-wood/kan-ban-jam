use crate::board::Board;
use std::path::PathBuf;

use warp::Filter;

#[tokio::main]
pub async fn server_main(board: &mut Board, path: &PathBuf) {

    board.list_items();
    board.save(path);
    
    println!("starting server...");
    
    let hello = warp::path!("hello" / String)
        .map(|name| {
        	println!("got a GET for the /hello endpoint!");
        	format!("Hello, {}!", name)
        });

    warp::serve(hello)
        .run(([192, 168, 1, 169], 3031))
        .await;
}