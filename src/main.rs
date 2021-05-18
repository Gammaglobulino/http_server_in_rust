use server::Server;
use http::HttpMethods;
use http::Request;
use website_handler::Websitehandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(Websitehandler);
}
