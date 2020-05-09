use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::sync::Arc;

use super::routes::Routes;
use super::http::{HttpMethod, Request};

pub struct Config<'a> {
    pub bind_path: &'a str,
    pub static_path: &'a str,
    pub routes: Routes<'a>,
}

struct RequestHandler;

impl RequestHandler {
    fn handle_request(&self, mut stream: TcpStream, static_path: String, routes: Arc<Routes>) {
        let mut buf = [0; 512];

        stream.read(&mut buf);
        let request = self.parse_request(&buf);

        let content = fs::read_to_string(format!(
            "{}{}",
            static_path,
            routes.get_route(request.url.as_ref())
        )).unwrap_or_else(|error| { error.to_string() });

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap_or_else(|error| {
            println!("{}", error);
            0
        });
        stream.flush().unwrap_or_else(|error| {
            println!("{}", error);
        });
    }

    fn parse_request(&self, request: &[u8]) -> Request {
        let request_str = String::from_utf8_lossy(request);
        println!("Request: {}", request_str);
        let blocks = request_str.split("\r\n").collect::<Vec<&str>>();
        let method = blocks[0].split(" ").collect::<Vec<&str>>();
        let http_method = match method[0] {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD" => HttpMethod::HEAD,
            "CONNECT" => HttpMethod::CONNECT,
            "TRACE" => HttpMethod::TRACE,
            "PATCH" => HttpMethod::PATCH,
            _ => HttpMethod::UNKNOWN,
        };

        let headers = if blocks.len() > 1 {
            blocks[1].to_string()
        } else {
            String::new()
        };

        let url = if method.len() > 1 {
            method[1].to_string()
        } else {
            String::new()
        };

        Request {
            method: http_method,
            url,
            headers,
            body: String::new(),
        }
    }
}

pub struct Ctchi {
    config: Config<'static>,
}

impl Ctchi {
    pub fn new(config: Config<'static>) -> Ctchi {
        Ctchi {
            config,
        }
    }

    pub fn start(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.config.bind_path)?;
        let routes = Arc::new(self.config.routes);
        let static_path = self.config.static_path.to_string();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let r = routes.clone();
            let s = static_path.clone();

            thread::spawn(|| {
                let handler = RequestHandler {};

                handler.handle_request(stream, s, r);
            });
        }
        Ok(())
    }
}