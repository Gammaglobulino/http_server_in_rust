use server::Server;
use http::HttpMethods;
use http::Request;
use website_handler::Websitehandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path=format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let puplic_path=env::var("PUBLIC PATH").unwrap_or(default_path);
    println!("The public path is: {}",puplic_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(Websitehandler::new(puplic_path));
}
