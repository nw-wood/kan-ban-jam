use crate::board::Board;
use std::path::PathBuf;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, oneshot};
use warp::{filters::ws::WebSocket, Filter};

use warp::ws::Message;

const SERVER_ADDR: [u8; 4] = [192, 168, 1, 169];
const SERVER_PORT: u16      = 3032;

const WEB_FOLDER: &str = "web/";

#[tokio::main]
pub async fn server_main(board: &mut Board, path: &PathBuf) {

    board.list_items();
    board.save(path);

    println!("starting server...");

    let (tx, rx) = oneshot::channel();

    //API routes

    /*let hi = warp::path("hello").and(warp::get().map(|| "hello")); //GET route that responds with "hello"

    let apis = hi;*/ //if there were more it'd be hi.or(bye).or(dink).or(donk)

    //Static content route

    let content = warp::fs::dir(WEB_FOLDER);
    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER))); //serve up the web app

    //serves up standard page that must contain js script

    let static_site = content.or(root); //combines content and root filter
                                                                                    //which is probably not needed? only root is needed prob

    //more learnings is leading me down the path of using javascript as a websocket client to the server
    //warp supports websocket stuff so this is probably the best way of handling the back and forth between client and server

    //the initial page served up by the rust server contains the javascript client code, and that code starts executing on the client end
    //the client then talks back to the server and starts saying it exists, that it's hungry for data, etc
    //the server takes this back talk and executes on it, and also responds with json payloads, or whatever

    //probably important to keep in consideration some stuff I tihnk is PROBBAAAABLY important
    //although not important right now, authentication _is_ important, but not until things actually work
    //the duration clients are considered for is probably important... some kind of "ping pong hey yes I'm here you're here we're here thing"

    //try to setup a websocket route instead of an ordinary one

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| { ws.on_upgrade(handle_websocket)
    });

    let routes = static_site.or(ws_route);

    let (_addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown((SERVER_ADDR, SERVER_PORT), async {
            rx.await.ok();
        });

    tokio::task::spawn(server);

    println!("press enter to shutdown");

    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);

    println!("poof!");

    let _ = tx.send(());

}

async fn handle_websocket(ws: WebSocket) {

    let (mut tx, mut rx) = ws.split();

    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(result) = rx.next().await {
            if let Ok(msg) = result {
                if msg.is_text() {
                    println!("recieved: {}", msg.to_str().unwrap());
                    msg_tx.send("server response!".to_string()).unwrap();
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(message) = msg_rx.recv().await {
            tx.send(Message::text(message)).await.unwrap();
        }
    });
}