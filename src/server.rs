use crate::board::Board;

use std::path::PathBuf;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use warp::ws::Message;
use warp::filters::ws::WebSocket;
use warp::Filter;

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
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));

    let static_site = content.or(root);

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
                    msg_tx.send("server response!".to_string()).unwrap();
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