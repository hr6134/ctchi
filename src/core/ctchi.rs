use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

pub struct Config<'a> {
    pub bind_path: &'a str,
    pub static_path: &'a str,
}

pub struct Ctchi<'a> {
    config: Config<'a>,
}

impl<'a> Ctchi<'a> {
    pub fn new(config: Config) -> Ctchi {
        Ctchi {
            config,
        }
    }

    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.config.bind_path)?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buf = [0; 512];

        // todo add exception handler
        stream.read(&mut buf).unwrap();
        let request = self.parse_request(&buf);

        // todo remove routes after testing
        // todo add routes as part of configuration
        let content = match request.url.as_ref() {
            "/" => fs::read_to_string(format!("{}{}", self.config.static_path, "/src/static/index.html")).unwrap(),
            "/blog" => fs::read_to_string(format!("{}{}", self.config.static_path, "/src/static/blog.html")).unwrap(),
            _ => fs::read_to_string(format!("{}{}", self.config.static_path, "/src/static/404.html")).unwrap(),
        };

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn parse_request(&self, request: &[u8]) -> Request {
        let request_str = String::from_utf8_lossy(request);
        let blocks = request_str.split("\r\n").collect::<Vec<&str>>();
        let method = blocks[0].split(" ").collect::<Vec<&str>>();
        let http_method = match method[0] {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("Dont know method {}", method[0]),
        };

        Request {
            method: http_method,
            url: method[1].to_string(),
            headers: blocks[1].to_string(),
            body: String::new(),
        }
    }
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}

struct Request {
    method: HttpMethod,
    url: String,
    headers: String,
    body: String,
}