use crate::board::Board;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{self, UnboundedSender};
use warp::filters::ws::WebSocket;
use warp::ws::Message;
use warp::Filter;

const SERVER_ADDR: [u8; 4] = [192, 168, 1, 169];
const SERVER_PORT: u16 = 3032;
const WEB_FOLDER: &str = "web/";

#[tokio::main]
pub async fn server_main(board: Arc<Mutex<Board>>, path: &PathBuf) {
    if let Ok(board_lock) = board.lock() {
        board_lock.list_items();
        board_lock.save(path);
    }

    let (tx, rx) = tokio::sync::oneshot::channel();

    let content = warp::fs::dir(WEB_FOLDER);
    let board_clone_a = Arc::clone(&board);
    let board_filter = warp::any().map(move || Arc::clone(&board_clone_a));

    // Shared state to store connected WebSocket clients
    let clients = Arc::new(Mutex::new(Vec::new()));
    let clients_filter = warp::any().map(move || Arc::clone(&clients));

    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));

    let static_site = content.or(root);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(board_filter)
        .and(clients_filter.clone())
        .map(|ws: warp::ws::Ws, board, clients| {
            ws.on_upgrade(move |socket| handle_websocket(socket, board, clients))
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

async fn handle_websocket(ws: WebSocket, board: Arc<Mutex<Board>>, clients: Arc<Mutex<Vec<UnboundedSender<Message>>>>) {
    let (mut tx, mut rx) = ws.split();
    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.push(msg_tx.clone());
        println!("New socket connection pushed to clients_guard");
    }

    let clients_clone = Arc::clone(&clients);

    // Handle incoming messages from the client
    tokio::spawn({
        let board_clone = Arc::clone(&board);
        async move {
            while let Some(result) = rx.next().await {
                if let Ok(msg) = result {
                    if msg.is_text() {
                        let msg = msg.to_str().unwrap().to_string();
                        if let Ok(result) = serde_json::from_str::<ClientResponse>(&msg) {
                            println!("Client message: {:?}", result);
                            let server_response = {
                                
                                let mut board_unlocked = board_clone.lock().unwrap();
                                match &result.command[..] {
                                    "ready" => Some(board_unlocked.serialized()),
                                    "add" => {
                                        board_unlocked.add_item(&result.args[0], &result.args[1]);
                                        Some(board_unlocked.serialized())
                                    }
                                    "demote" => {
                                        board_unlocked.demote_item(&result.args[0]);
                                        Some(board_unlocked.serialized())
                                    }
                                    "promote" => {
                                        board_unlocked.promote_item(&result.args[0]);
                                        Some(board_unlocked.serialized())
                                    }
                                    "edit_content" => {
                                        board_unlocked.update_item(&result.args[0], &result.args[1]);
                                        Some(board_unlocked.serialized())
                                    }
                                    "remove" => {
                                        board_unlocked.remove_item(&result.args[0]);
                                        Some(board_unlocked.serialized())
                                    }
                                    _ => {
                                        println!("Unknown input from the client");
                                        None
                                    }
                                }
                            };

                            if let Some(response) = server_response {
                                let clients_guard = clients_clone.lock().unwrap();
                                for client in clients_guard.iter() {
                                    if let Err(e) = client.send(Message::text(response.clone())) {
                                        eprintln!("Failed to send message: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    tokio::spawn({
        let board_clone = Arc::clone(&board);
        async move {
            while let Some(_) = msg_rx.recv().await {
                let serialized = {
                    let board_unlocked = board_clone.lock().unwrap();
                    board_unlocked.serialized()
                };

                if let Err(e) = tx.send(Message::text(&serialized)).await {
                    eprintln!("Failed to send message: {}", e);
                }
            }
        }
    });
}