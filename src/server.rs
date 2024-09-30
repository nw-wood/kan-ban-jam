use crate::board::Board;
use std::path::PathBuf;
//use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::sync::oneshot;

// use futures_util::future::TryFutureExt;

use warp::Filter;

const SERVER_ADDR: [u8; 4] = [192, 168, 1, 169];
const SERVER_PORT: u16      = 3032;

#[tokio::main]
pub async fn server_main(board: &mut Board, path: &PathBuf) {

    board.list_items();
    board.save(path);

    println!("starting server...");

    let (tx, rx) = oneshot::channel();

    let route_root = warp::path::end().map(|| "hello from root");

    let route_shutdown = warp::path("shutdown").and(warp::get()).map(|| {
        "shutdown!"
    });

    let routes = route_root.or(route_shutdown);

    let (_addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown((SERVER_ADDR, SERVER_PORT), async {
            rx.await.ok();
        });

    tokio::task::spawn(server); //<-- spawns server in its own thread I assume

    println!("press enter to shutdown");

    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);

    println!("poof!");

    let _ = tx.send(()); //<-- sends unit struct to reciever which is held in server thread?

    //probably should wait until the server thread has shutdown before ending the process
    //std::process::exit(0);


}

// ? brain time

//so probably want to be able to autheticate just exactly who is allowed to do what
//maybe figure out oath, and see if that gives at least a valid email of a user that can be compared to a list of configured users we allow to do write operations on the server
// write operations would be operations that 