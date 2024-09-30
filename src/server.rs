use crate::board::Board;
use std::path::PathBuf;
use tokio::sync::oneshot;
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


    //need to figure out some routes for POST instead of GET, and figure for what things are actually important
    //need to serve up an actual index.html from the root directory, and that needs to be running some JS
    //the JS from the website needs to be what does the POSTing back to the server
    // p r o b a b l y

    //API routes

    let hi = warp::path("hello").and(warp::get().map(|| "hello")); //GET route that responds with "hello"

    let apis = hi; //if there were more it'd be hi.or(bye).or(dink).or(donk)

    //Static content route

    let content = warp::fs::dir(WEB_FOLDER); //sets WEB_FOLDER as the working dir?
    let root = warp::get()  //don't think this get() is needed?
        .and(warp::path::end())                                                   //for the 'end' path  
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));         //sends back a file instead of some .map(|| pred);

    let static_site = content.or(root); //combines content and root filter
                                                                                    //which is probably not needed? only root is needed prob

    let routes = apis.or(static_site);

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