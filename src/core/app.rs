use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::sync::Arc;

use super::routes::Routes;
use super::http::{HttpMethod, Request};
use super::thread_pool::{ThreadPool};
use crate::core::config::get_configuration;

struct RequestHandler;

fn read_static(file_pth: &str) -> impl Fn(&str) -> String + '_ {
    move |pref| -> String {
        use std::fs;
        let content = fs::read_to_string(format!("{}/{}", pref, file_pth))
            .unwrap_or_else(|error| { error.to_string() });
        content
    }
}

impl RequestHandler {
    fn handle_request(&self, stream: TcpStream, routes: Arc<Routes>) {
        let mut request_input = Vec::new();
        let mut reader = BufReader::new(stream);

        for line in reader.by_ref().lines() {
            let l = line.unwrap();
            request_input.extend_from_slice(l.as_bytes());
            if l == String::from("") {
                break;
            }
        };

        let request = self.parse_request(&request_input);

        let config_reader = get_configuration();
        let config = config_reader.inner.lock().unwrap();

        let prefix = &config.static_uri_pref;
        let content = if request.url.starts_with(prefix) {
            read_static(&request.url)(config.base_path.as_ref())
        } else {
            drop(config);
            (routes.get_route(request.url.as_ref()).render_action)(request.url.as_ref())
        };

        let response = format!(
            "HTTP/1.1 200 OK\r\n\r\n{}",
            content
        );
        println!("Response: {}", response);

        let mut reader_stream = reader.into_inner();
        let result = reader_stream.write(response.as_bytes());
        result.unwrap_or_else(|error| {
            println!("{}", error);
            0
        });

        reader_stream.flush().unwrap_or_else(|error| {
            println!("{}", error);
        });
    }

    /// Parse stream of bytes in Request object.
    /// Gets URI, HTTP method, headers and body.
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

        let mut url = if method.len() > 1 {
            method[1].to_string()
        } else {
            String::new()
        };

        if !url.ends_with("/")
            && !url.ends_with(".css")
            && !url.ends_with(".js")
            && !url.ends_with(".jpg")
        {
            url = format!("{}/", url);
        }

        Request {
            method: http_method,
            url,
            headers,
            body: String::new(),
        }
    }
}

pub struct Ctchi {
    routes: Routes,
}

impl Ctchi {
    /// Create new application with specified configuration.
    ///
    /// # Arguments:
    /// * `config` - configuration for application. `ctchi::core::ctchi::Config`
    ///
    pub fn new(routes: Routes) -> Ctchi {
        Ctchi {
            routes
        }
    }

    /// Start configured application. Now it will lister for specified ip:port
    /// and respond for request if URI is in routes
    ///
    /// # Examples:
    ///
    /// ```rust,ignore
    ///   use ctchi::core::ctchi::{Config, Ctchi};
    ///   use ctchi::core::routes::Routes;
    ///
    ///   let mut routes = Routes::new();
    ///   routes.add_route("/", "/src/pages.static/index.html");
    ///
    ///   let configuration = Config {
    ///        bind_path: "127.0.0.1:8080",
    ///        base_path: "/var/www/",
    ///        routes,
    ///   };
    ///
    ///   let server = Ctchi::new(configuration);
    ///   server.start();
    /// ```
    pub fn start(self) -> std::io::Result<()> {
        let config_reader = get_configuration();
        let config = config_reader.inner.lock().unwrap();

        let listener = TcpListener::bind(&config.bind_path)?;
        drop(config);
        let routes = Arc::new(self.routes);

        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let r = routes.clone();

            pool.execute(|| {
                let handler = RequestHandler {};

                handler.handle_request(stream, r);
            });
        }
        Ok(())
    }
}