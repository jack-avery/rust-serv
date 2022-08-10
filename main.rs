mod serv;
use serv::api::{echo, HTTPEndpointHandler};

fn main() {
    let mut endpoints = HTTPEndpointHandler::new();
    endpoints.add("/echo", echo);

    endpoints.serve(8080);
}
