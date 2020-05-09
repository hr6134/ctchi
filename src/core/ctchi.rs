use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::sync::Arc;

use super::routes::Routes;
use super::http::{HttpMethod, Request};

pub struct Config<'a> {
    pub bind_path: &'a str,
    pub base_path: &'a str,
    pub routes: Routes<'a>,
}

struct RequestHandler;

impl RequestHandler {
    fn handle_request(&self, mut stream: TcpStream, config: Arc<Config>) {
        let mut buf = [0; 512];

        stream.read(&mut buf);
        let request = self.parse_request(&buf);

        let content = fs::read_to_string(format!(
            "{}{}",
            config.base_path,
            config.routes.get_route(request.url.as_ref())
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
        let http_method = HttpMethod::parse(method[0]);

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
        let config = Arc::new(self.config);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let c = config.clone();

            thread::spawn(|| {
                let handler = RequestHandler {};

                handler.handle_request(stream, c);
            });
        }
        Ok(())
    }
}