mod server;
use server::Server;

mod http;
use http::Request;
use http::HTTPMethod;

fn main() {
    let ipaddress = String::from("127.0.0.1:8080");
    let server = Server::new(ipaddress);
    server.run();
}



