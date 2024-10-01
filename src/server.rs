use crate::board::Board;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use warp::ws::Message;
use warp::filters::ws::WebSocket;
use warp::Filter;

const SERVER_ADDR: [u8; 4] = [192, 168, 1, 169];
const SERVER_PORT: u16      = 3032;

const WEB_FOLDER: &str = "web/";

#[tokio::main]
pub async fn server_main(board: Arc<Mutex<&mut Board>>, path: &PathBuf) {

    loop {
        if let Ok(board_lock) = board.try_lock() {
            board_lock.list_items();
            board_lock.save(path);
            break;
        } else {
            println!("")
        }
    }

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

    let sh = SERVER_ADDR;
    println!("url: http://{}.{}.{}.{}:{}/", sh[0], sh[1], sh[2], sh[3], SERVER_PORT);
    println!("press enter to shutdown");

    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);

    println!("poof!");

    let _ = tx.send(());

}

#[derive(Deserialize, Debug)]
struct ClientResponse {
    value: String,
}

#[derive(Serialize, Debug)]
struct ServerResponse {
    value: String,
}

impl ServerResponse { 
    fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

async fn handle_websocket(ws: WebSocket) {

    let (mut tx, mut rx) = ws.split();

    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(result) = rx.next().await {
            if let Ok(msg) = result {
                if msg.is_text() {
                    let msg = msg.to_str().unwrap().to_string();

                    let response = serde_json::from_str::<ClientResponse>(msg.as_str());

                    let mut server_response: String = String::new();

                    if let Ok(result) = response {
                        match result.value.as_str() {
                            "ready" => server_response = "TODO: response with the board serialized as json".to_string(),
                            _ => server_response = "unknown request".to_string(),
                        }
                    }

                    let server_response = ServerResponse::new(server_response);

                    if let Ok(json) = serde_json::to_string(&server_response) {
                        println!("received from client: {}", msg);
                        println!("sending back: {json}");
                        msg_tx.send(json).unwrap();
                    } else {
                        println!("issue serializing response to the client");
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