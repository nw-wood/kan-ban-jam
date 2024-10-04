use crate::board::Board;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use warp::ws::Message;
use warp::filters::ws::WebSocket;
use warp::Filter;

const SERVER_ADDR: [u8; 4] = [192, 168, 1, 169];
const SERVER_PORT: u16      = 3032;

const WEB_FOLDER: &str = "web/";

#[tokio::main]
pub async fn server_main(board: Arc<Mutex<Board>>, path: &PathBuf) {

    if let Ok(board_lock) = board.lock() {
        board_lock.list_items();
        board_lock.save(path);  
    }

    let (tx, rx) = oneshot::channel();

    //API routes

    /*let hi = warp::path("hello").and(warp::get().map(|| "hello")); //GET route that responds with "hello"
    let apis = hi;*/ //if there were more it'd be hi.or(bye).or(dink).or(donk)

    let content = warp::fs::dir(WEB_FOLDER);

    let board_clone_a = Arc::clone(&board);

    let board_filter = warp::any().map(move || Arc::clone(&board_clone_a));

    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));

    let static_site = content.or(root);

    let ws_route = warp::path("ws") 
        .and(warp::ws())
        .and(board_filter)
        .map(|ws: warp::ws::Ws, board| {
            ws.on_upgrade(move |socket| handle_websocket(socket, board))
        });

    let routes = static_site.or(ws_route);

    let (_addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown((SERVER_ADDR, SERVER_PORT), async {
            rx.await.ok();
        });

    tokio::task::spawn(server);

    let sh = SERVER_ADDR;
    println!("url: http://{}.{}.{}.{}:{}/", sh[0], sh[1], sh[2], sh[3], SERVER_PORT);
    println!("press enter to shutdown");

    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);

    loop {
        if let Ok(board_lock) = board.try_lock() {
            board_lock.list_items();
            board_lock.save(path);
            break;
        }
    }

    println!("poof!");

    let _ = tx.send(());

}

#[derive(Deserialize, Debug)]
struct ClientResponse {
    command: String,
    args: Vec<String>,
}

async fn handle_websocket(ws: WebSocket, board: Arc<Mutex<Board>>) {

    let (mut tx, mut rx) = ws.split();

    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(result) = rx.next().await {
            if let Ok(msg) = result {
                if msg.is_text() {
                    let msg = msg.to_str().unwrap().to_string();

                    let response = serde_json::from_str::<ClientResponse>(msg.as_str());

                    if let Ok(result) = response {

                        println!("client message: {result:?}");

                        let mut server_response: String = String::new();

                        if let Ok(mut board_unlocked) = board.lock() {

                            match &result.command.as_str() {

                                &"ready" => server_response = board_unlocked.serialized(),
                                &"add" => {
                                    board_unlocked.add_item(result.args[0].as_str(), result.args[1].as_str());
                                    server_response = board_unlocked.serialized();
                                }
                                _ => {
                                    println!("unknown input from the client");
                                    //_server_response = serde_json::to_string(&response).unwrap();
                                }
                            }
                        }
                        msg_tx.send(server_response).unwrap();
                    }
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