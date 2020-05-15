use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::sync::Arc;
use std::env::current_dir;

use super::routes::Routes;
use super::http::{HttpMethod, Request};
use super::thread_pool::{ThreadPool};

#[derive(Debug)]
pub struct Config {
    pub bind_path: String,
    pub base_path: String,
    pub static_uri_pref: String,
}

impl Config {
    pub fn new() -> Config {
        Config::parse_config("/etc/ctchi/conf.txt")
    }

    fn parse_config(path: &str) -> Config {
        let mut bind_path ="127.0.0.1:8080";

        let config_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => String::new()
        };

        let mut templates_dir = format!(
            "{}{}",
            current_dir().unwrap().to_str().unwrap(),
            "/src/pages/"
        );

        let lines = config_content.split("\n").collect::<Vec<&str>>();
        for l in lines {
            let cols = l.split("=").collect::<Vec<&str>>();
            if cols[0] == "bind_path" {
                bind_path = cols[1];
            }
            if cols[0] == "base_path" {
                templates_dir = cols[1].to_string();
            }
        }

        Config {
            bind_path: bind_path.to_string(),
            base_path: templates_dir,
            static_uri_pref: "/static/".to_string(),
        }
    }
}

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
    fn handle_request(&self, mut stream: TcpStream, config: Arc<Config>, routes: Arc<Routes>) {
        let mut buf = [0; 512];

        stream.read(&mut buf);
        let request = self.parse_request(&buf);

        let prefix = &config.static_uri_pref;
        let content = if request.url.starts_with(prefix) {
            read_static(&request.url)(config.base_path.as_ref())
        } else {
            (routes.get_route(request.url.as_ref()).render_action)(config.base_path.as_ref())
        };

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap_or_else(|error| {
            println!("{}", error);
            0
        });
        stream.flush().unwrap_or_else(|error| {
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
    config: Config,
    routes: Routes,
}

impl Ctchi {
    /// Create new application with specified configuration.
    ///
    /// # Arguments:
    /// * `config` - configuration for application. `ctchi::core::ctchi::Config`
    ///
    pub fn new(config: Config, routes: Routes) -> Ctchi {
        Ctchi {
            config,
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
        let listener = TcpListener::bind(&self.config.bind_path)?;
        let config = Arc::new(self.config);
        let routes = Arc::new(self.routes);

        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let c = config.clone();
            let r = routes.clone();

            pool.execute(|| {
                let handler = RequestHandler {};

                handler.handle_request(stream, c, r);
            });
        }
        Ok(())
    }
}