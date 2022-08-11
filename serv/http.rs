use std::io::{Error, Read, Write};
use std::net::TcpStream;

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub loc: String,
    query: Vec<Query>,
}

#[derive(Clone)]
pub struct Response {
    version: String,
    pub code: Code,
    pub headers: Vec<Header>,
    pub body: String,
}

#[derive(Clone)]
pub enum Code {
    Ok,               // 200
    MalformedRequest, // 400
    Forbidden,        // 402
    NotFound,         // 404
    InternalError,    // 500
}

impl Code {
    pub fn to_str(&self) -> &str {
        match self {
            Code::Ok => "200 OK",
            Code::MalformedRequest => "400 MALFORMED REQUEST",
            Code::Forbidden => "402 FORBIDDEN",
            Code::NotFound => "404 NOT FOUND",
            Code::InternalError => "500 INTERNAL ERROR",
        }
    }
}

#[derive(Clone)]
pub struct Query {
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Request {
    pub fn new(method: String, path: String, version: String, headers: Vec<Header>) -> Request {
        let query: Vec<Query> = Vec::new();
        let mut req = Request {
            method,
            path,
            version,
            headers,
            loc: String::new(),
            query,
        };

        let path = req.path.clone();

        if let Some(p) = path.split_once('?') {
            req.loc = p.0.to_string();

            let qry = p.1.split('&');

            for q in qry {
                if let Some(a) = q.split_once('=') {
                    if !a.0.to_string().is_empty() && !a.1.to_string().is_empty() {
                        req.query.push(Query {
                            key: a.0.to_string(),
                            value: a.1.to_string(),
                        });
                    }
                }
            }
        } else {
            req.loc = path;
        }

        req
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{} {} {} ({} headers)",
            self.method,
            self.path,
            self.version,
            self.headers.len(),
        );
    }

    pub fn get_header(&self, key: &str) -> Option<Header> {
        let hiter = self.headers.clone().into_iter();
        for h in hiter {
            if h.key == key {
                return Some(h);
            }
        }
        None
    }

    pub fn get_param(&self, key: &str) -> Option<String> {
        let qiter = self.query.clone().into_iter();
        for q in qiter {
            if q.key == key {
                return Some(q.value);
            }
        }
        None
    }
}

impl Response {
    pub fn new(code: Code) -> Response {
        let headers: Vec<Header> = Vec::new();
        Response {
            version: "HTTP/1.1".to_string(),
            code,
            headers,
            body: String::new(),
        }
    }

    pub fn new_ok() -> Response {
        Self::new(Code::Ok)
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.version, self.code.to_str(),)
    }

    pub fn to_http(&self) -> String {
        let mut res = format!("{} {}\n", self.version, self.code.to_str());

        let hiter = self.headers.clone().into_iter();
        for h in hiter {
            res += format!("{}\n", h.to_string()).as_str();
        }

        res += "\n";
        res += self.body.as_str();
        res
    }

    pub fn mod_header(&mut self, key: &str, value: &str) {
        let hiter = self.headers.clone().into_iter();
        for mut h in hiter {
            if h.key == key {
                h.value = value.to_string();
                return;
            }
        }
        self.headers.push(Header {
            key: key.to_string(),
            value: value.to_string(),
        });
    }
}

impl Header {
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

pub fn parse_tcpstream(mut stream: &TcpStream) -> Result<Request, Error> {
    let mut buf = [0u8; 4096];

    match stream.read(&mut buf) {
        Ok(_) => {
            let req = String::from_utf8_lossy(&buf);
            let iter = req.to_string();
            let mut iter = iter.split('\n');

            let mut method = String::new();
            let mut path = String::new();
            let mut version = String::new();
            let mut headers: Vec<Header> = Vec::new();

            if let Some(startline) = iter.next() {
                let sliter = String::from(startline);
                let mut sliter = sliter.trim().split(' ');

                method = sliter.next().unwrap().to_string();
                path = sliter.next().unwrap().to_string();
                version = sliter.next().unwrap().to_string();
            }

            for header in iter {
                let head = header.to_string();
                let head = head.trim();
                let mut head = head.split(": ");

                let mut key = String::new();
                let mut value = String::new();

                if let Some(k) = head.next() {
                    key = k.to_string();
                }
                if let Some(v) = head.next() {
                    value = v.to_string();
                }

                if !key.is_empty() && !value.is_empty() {
                    headers.push(Header { key, value });
                }
            }

            Ok(Request::new(method, path, version, headers))
        }
        Err(e) => Err(e),
    }
}

pub fn respond_to_tcpstream(mut stream: &TcpStream, mut with: Response) -> Result<usize, Error> {
    with.mod_header("Content-Length", format!("{}", with.body.len()).as_str());
    let resp = with.to_http();
    let resp = resp.as_bytes();
    stream.write(resp)
}
