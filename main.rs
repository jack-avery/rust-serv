mod serv;
use serv::{api, http};
use std::num::ParseIntError;

fn main() {
    let mut endpoints = api::HTTPEndpointHandler::new();
    endpoints.add("/echo", example_echo);
    endpoints.add("/mult", example_mult);
    endpoints.add("/passurl", example_passurl);
    endpoints.add("/passhead", example_passhead);

    endpoints.serve(8080);
}

fn example_echo(req: &http::Request) -> http::Response {
    let mut res = http::Response::new_ok();

    res.mod_header("Content-Type", "text/html; charset=utf-8");

    res.body = req.clone().path;

    res
}

fn example_mult(req: &http::Request) -> http::Response {
    let mut res = http::Response::new_ok();

    if let Some(n) = req.get_param("number") {
        let num: Result<i32, ParseIntError> = n.trim().parse();

        match num {
            Ok(i) => res.body = (i * 2).to_string(),
            Err(e) => {
                res = api::gen400(e.to_string());
            }
        }
    }

    res
}

fn example_passurl(req: &http::Request) -> http::Response {
    let mut res = http::Response::new_ok();

    if let Some(n) = req.get_param("foo") {
        if n == "bar".to_string() {
            res.body = "ok".to_string();
        }
    } else {
        res = api::gen402(req);
    }

    res
}

fn example_passhead(req: &http::Request) -> http::Response {
    let mut res = http::Response::new_ok();

    if let Some(n) = req.get_header("Foo") {
        if n == "Bar".to_string() {
            res.body = "ok".to_string();
        }
    } else {
        res = api::gen402(req);
    }

    res
}
