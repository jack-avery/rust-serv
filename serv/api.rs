use crate::serv::http;

use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Clone)]
struct Endpoint {
    loc: String,
    func: fn(&http::Request) -> http::Response,
}

#[derive(Clone)]
pub struct HTTPEndpointHandler {
    endpoints: Vec<Endpoint>,
}

impl HTTPEndpointHandler {
    pub fn new() -> HTTPEndpointHandler {
        let endpoints: Vec<Endpoint> = Vec::new();
        HTTPEndpointHandler { endpoints }
    }

    fn handle_request(&self, stream: TcpStream) {
        match http::parse_tcpstream(&stream) {
            Ok(r) => {
                http::respond_to_tcpstream(&stream, self.process(r));
            }
            Err(e) => {
                println!("Invalid request sent: {}", e);
            }
        };
    }

    fn process(&self, req: http::Request) -> http::Response {
        for e in self.endpoints.iter() {
            if req.loc == e.loc {
                return (e.func)(&req);
            }
        }
        gen404()
    }

    pub fn add(&mut self, loc: &str, func: fn(&http::Request) -> http::Response) {
        self.endpoints.push(Endpoint {
            loc: loc.to_string(),
            func,
        })
    }

    pub fn serve(&self, port: u32) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        println!("Listening for connections on port {}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let endpoints = self.clone();
                    thread::spawn(move || endpoints.handle_request(stream));
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
    }
}

pub fn echo(req: &http::Request) -> http::Response {
    let mut res = http::Response::new(http::Code::Ok);

    res.mod_header("Content-Type", "text/html; charset=utf-8");

    res.body = req.clone().path;

    res
}

pub fn gen402(req: &http::Request) -> http::Response {
    let mut res = http::Response::new(http::Code::Forbidden);

    res.mod_header("Content-Type", "text/html; charset=utf-8");

    res.body = format!("unauthorized for access to resource {}", req.loc);

    res
}

pub fn gen404() -> http::Response {
    let mut res = http::Response::new(http::Code::NotFound);

    res.mod_header("Content-Type", "text/html; charset=utf-8");

    res.body = res.to_string();

    res
}

pub fn gen500(err: String) -> http::Response {
    let mut res = http::Response::new(http::Code::InternalError);

    res.mod_header("Content-Type", "text/html; charset=utf-8");

    res.body = format!("{}\n{}", res.to_string(), err);

    res
}
