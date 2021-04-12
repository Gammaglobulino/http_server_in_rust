use server::Server;
use http::HttpMethods;
use http::Request;

mod http;
mod server;


fn main() {
    let get = HttpMethods::GET;
    let post = HttpMethods::POST;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
