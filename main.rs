mod serv;
use serv::{api, http};
use std::num::ParseIntError;

fn main() {
    let mut endpoints = api::HTTPEndpointHandler::new();
    endpoints.add("/echo", echo);
    endpoints.add("/mult", example_multbytwo);

    endpoints.serve(8080);
}

pub fn echo(req: &http::Request) -> http::Response {
    let mut res = http::Response::new(http::Code::Ok);

    res.mod_header("Content-Type", "text/html; charset=utf-8");

    res.body = req.clone().path;

    res
}

fn example_multbytwo(req: &http::Request) -> http::Response {
    let mut res = http::Response::new(http::Code::Ok);

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
