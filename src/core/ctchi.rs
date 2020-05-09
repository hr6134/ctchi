use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::collections::HashMap;

pub struct Routes<'a> {
    routes: HashMap<&'a str, &'a str>,
}

impl<'a> Routes<'a> {
    pub fn new() -> Routes<'a> {
        Routes {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, uri: &'a str, file: &'a str) {
        self.routes.insert(uri, file);
    }

    pub fn get_route(&self, uri: &'a str) -> &str {
        self.routes.get(uri).unwrap_or(&"/404")
    }
}

pub struct Config<'a> {
    pub bind_path: &'a str,
    pub static_path: &'a str,
    pub routes: Routes<'a>,
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

        stream.read(&mut buf);
        let request = self.parse_request(&buf);

        let content = fs::read_to_string(format!(
            "{}{}",
            self.config.static_path,
            self.config.routes.get_route(request.url.as_ref())
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

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
    PATCH,
    UNKNOWN,
}

struct Request {
    method: HttpMethod,
    url: String,
    headers: String,
    body: String,
}