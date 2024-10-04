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

//TODO: spend time with async as a keyword - make more simple examples work
//TODO: spend more time with Paths and PathBuf and learn more of their methods as they are fairly common
pub async fn server_main(board: Arc<Mutex<Board>>, path: &PathBuf) {
    //TODO: although not explicitly written here, there is an await after this locking func I believe, and I should spend more time understanding await
    if let Ok(board_lock) = board.lock() {
        board_lock.list_items();
        board_lock.save(path);
    }
    //TODO: I believe this is a mspc impl that gets dropped immediately after use, but will have to read into it
    let (tx, rx) = tokio::sync::oneshot::channel();

    //TODO: Spend more time setting up and tinkering with less or even more complicated warp filter combinations
    let content = warp::fs::dir(WEB_FOLDER);

    //TODO: Reread the rust book documentation on some of the available smart pointers like Rc and RefCells and Arc
    //TODO: Understand how atomics works better because right now it's an 'lol thread safe variable'
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
        //TODO: I'm not entirely sure how this ws return implementation fires back the ws to the map - I need to learn what 'upgrading' is
        .map(|ws: warp::ws::Ws, board, clients| {
            ws.on_upgrade(move |socket| handle_websocket(socket, board, clients))
        });

    let routes = static_site.or(ws_route);

    //TODO: test and make up examples of Trait bound implementations because this is excessively difficult to interpret
    //  the bind with graceful shutdown method has so many of them I really couldn't say what every single one implied all together
    //  some of the trait bounds here are fairly common and I should learn better usage anyways (clone, send, sync, 'static)
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

    //TODO: setup small arbitrary examples of mspcs, oneshots, broadcast channels and so on to experiment and learn
    let _ = tx.send(());
}

struct Client {
    id: usize,
    sender: UnboundedSender<Message>,
}

#[derive(Deserialize, Debug)]
struct ClientResponse {
    command: String,
    args: Vec<String>,
}

fn remove_stale_clients(clients: &Arc<Mutex<Vec<Client>>>) {
    let mut clients = clients.lock().unwrap();
    clients.retain(|client| {
        //TODO: look into message implementaions for warp filters (it's various methods and usage is kind of a mystery)
        client.sender.send(Message::ping(vec![])).is_ok()
    });
}

async fn handle_websocket(ws: WebSocket, board: Arc<Mutex<Board>>, clients: Arc<Mutex<Vec<Client>>>) {
    //TODO: understand sinks and streams in this context, the spit function here seperates them, and I believe they basically refer to the transmit and recv
    //  end of message passing functionality
    let (mut tx, mut rx) = ws.split();
    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();
    
    // Generate a unique client ID
    let client_id = {
        let clients = clients.lock().unwrap();
        clients.len()
    };

    // Add new client to the list
    {
        let mut clients_guard = clients.lock().unwrap();
        //TODO: try to think like this more often... my thoughts were the message senders into a vector, but making them into an object
        //  and putting those into a vector makes a lot more sense, and makes them more manageable since methods could be implemented on
        //  the clients structure now
        clients_guard.push(Client {
            id: client_id,
            sender: msg_tx.clone(),
        });
        println!("New socket connection (id: {}). Total clients: {}", client_id, clients_guard.len());
    }

    let clients_clone = Arc::clone(&clients);

    // Handle incoming messages and cleanup in a single task
    tokio::spawn({
        let board_clone = Arc::clone(&board);
        async move {
            while let Some(result) = rx.next().await {
                match result {
                    Ok(msg) => {
                        if msg.is_text() {
                            let msg = msg.to_str().unwrap().to_string();
                            if let Ok(result) = serde_json::from_str::<ClientResponse>(&msg) {
                                println!("Client message from id {}: {:?}", client_id, result);
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
                                            println!("Unknown input from client {}", client_id);
                                            None
                                        }
                                    }
                                };

                                // If we have a response, broadcast it to all clients
                                if let Some(response) = server_response {
                                    // Clean up stale clients before broadcasting
                                    remove_stale_clients(&clients_clone);
                                    
                                    let clients_guard = clients_clone.lock().unwrap();
                                    for client in clients_guard.iter() {
                                        if let Err(e) = client.sender.send(Message::text(response.clone())) {
                                            eprintln!("Failed to send message to client {}: {}", client.id, e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("WebSocket error for client {}: {}", client_id, e);
                        break;
                    }
                }
            }

            // Clean up the disconnected client
            let mut clients = clients_clone.lock().unwrap();
            let client_count_before = clients.len();
            clients.retain(|client| {
                if client.id == client_id {
                    // Remove this specific client
                    false
                } else {
                    // Keep other clients if they're still connected
                    client.sender.send(Message::ping(vec![])).is_ok()
                }
            });
            let client_count_after = clients.len();
            let removed_count = client_count_before - client_count_after;
            println!("Client {} disconnected. Removed {} clients. Remaining clients: {}", 
                    client_id, removed_count, client_count_after);
        }
    });

    // Send board state to the client periodically
    tokio::spawn({
        let board_clone = Arc::clone(&board);
        async move {
            while let Some(_) = msg_rx.recv().await {
                let serialized = {
                    let board_unlocked = board_clone.lock().unwrap();
                    board_unlocked.serialized()
                };

                if let Err(e) = tx.send(Message::text(&serialized)).await {
                    eprintln!("Failed to send message to client {}: {}", client_id, e);
                    break;  // Exit the loop if we can't send to this client
                }
            }
        }
    });
}