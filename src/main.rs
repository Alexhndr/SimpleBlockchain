mod time;
mod net_location;
mod proof;
mod settings;
mod node;
mod nodes;
mod transaction;
mod transactions;
mod block;
mod chain;
mod requests;
mod blockchain;
mod routing;

use std::io::Result;
use std::sync::{Arc, Mutex};
use actix_web::{App, HttpServer};

use settings::Settings;
use blockchain::Blockchain;

#[actix_rt::main]
async fn main() -> Result<()> {
    let settings = Settings::new();
    let blockchain = Blockchain::new(settings.socket_address());
    
    println!("Net location: {}", blockchain.net_location());
    
    let blockchain = Arc::new(Mutex::new(blockchain));
    let http_server = HttpServer::new(move || {
            App::new()
            .data(blockchain.clone())
            .configure(routing::initialize)
        });
    http_server.bind(settings.socket_address())?.run().await
}
