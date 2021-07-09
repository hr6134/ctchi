use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::sync::Arc;
use std::env::current_dir;


use super::routes::Routes;
use super::http::{HttpMethod, Request};
use super::thread_pool::{ThreadPool};
use std::path::PathBuf;


pub struct Config {
    pub bind_path: &'static str,
    pub base_path: &'static str,
    pub static_uri_pref: &'static str,
}

impl Config {
    pub fn new() -> Config {
        Config::parse_config("/etc/ctchi/ctchi.properties")
    }

    fn parse_config(path: &str) -> Config {
        let config = match fs::read_to_string(path) {
            Ok(config) => config,
            Err(_) => "".to_string(),
        };

        let lines = config.split("\n").collect::<Vec<&str>>();
        // todo get configuration

        let base_path = lines[0];

//        let path = match current_dir() {
//            Ok(p) => p,
//            Err(_) => PathBuf::from(""),
//        };
//        let base_path = path.to_str();
//        let base_path = format!("{}{}", current_dir().unwrap().to_str().unwrap(), "/src/static");

        Config {
            base_path: base_path,
//            base_path: "",
            bind_path: "127.0.0.1:8080",
            static_uri_pref: "/css/",
        }
    }
}

struct RequestHandler;

fn read_static(file_pth: &str) -> impl Fn(&str) -> String + '_ {
    move |pref| -> String {
        let content = fs::read_to_string(format!("{}/{}", pref, file_pth))
            .unwrap_or_else(|error| { error.to_string() });
        content
    }
}

impl RequestHandler {
    fn handle_request(&self, stream: TcpStream, routes: Arc<Routes>) {
        let mut reader = BufReader::new(stream);

        let request = self.parse_request(&mut reader);

        log::info!("Request: {:?} {}", request.method, request.url);
        if !request.body.is_empty() {
            log::info!("{}", request.body);
        }

        let config_reader = get_configuration();
        let config = config_reader.inner.lock().unwrap();
        let tmp_base_path = config.base_path.to_string();
        let prefix = config.static_uri_pref.to_string();
        drop(config);

        let content = if request.url.starts_with(&prefix) {
            read_static(&request.url)(tmp_base_path.as_str())
        } else {
            (routes.get_route(request.url.as_ref()).render_action)(config.base_path)
        };

        let response = format!(
            "HTTP/1.1 200 OK\r\n\r\n{}",
            content
        );
        log::info!("Response: {}", response);

        let mut reader_stream = reader.into_inner();
        let result = reader_stream.write(response.as_bytes());
        result.unwrap_or_else(|error| {
            log::info!("{}", error);
            0
        });

        reader_stream.flush().unwrap_or_else(|error| {
            log::info!("{}", error);
        });
    }

    /// Parse stream of bytes in Request object.
    /// Gets URI, HTTP method, headers and body.
    fn parse_request(&self, reader: &mut BufReader<TcpStream>) -> Request {
        let mut lines = reader.by_ref().lines();

        // get method line, it should be in every request, so unwrapping is safe
        // fixme thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value',
        // /Users/glotitude/.cargo/registry/src/github.com-1ecc6299db9ec823/ctchi-0.19.0/src/core/app.rs:69:40
        // for some reason
        let method_line = lines.next().unwrap().unwrap();
        let method = method_line.split(" ").collect::<Vec<&str>>();
        let http_method = HttpMethod::parse(method[0]);

        let mut headers = HashMap::<String, String>::new();
        for line in lines {
            let l = line.unwrap();

            // fixme we will miss body in this case
            if l == String::from("") {
                break;
            }

            let parts = l.split(":").collect::<Vec<&str>>();
            if parts.len() == 2 {
                headers.insert(parts[0].to_string(), parts[1].to_string());
            }
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
    config: Config,

    routes: Routes,
}

impl Ctchi {
    /// Create new application with specified routes.
    ///
    /// Configuration gets by `ctchi::core::ctchi::get_configuration()` singleton.
    ///
    /// # Arguments:
    /// * `routes` - list of routes. `ctchi::core::ctchi::Routes`
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
    /// #![feature(concat_idents)]
    /// #[macro_use]
    /// extern crate ctchi;
    ///
    /// use ctchi::core::app::Ctchi;
    /// use ctchi::core::routes::{Routes, Route};
    ///
    /// use ctchi_codegen::route;
    ///
    /// #[route("/")]
    /// fn index() -> String {
    ///     render!("index.html")
    /// }
    ///
    /// fn main() {
    ///     let mut routes = Routes::new();
    ///     // add route to your controller
    ///     routes.add_route(routes!(index)());
    ///
    ///     // create and run local server
    ///     let server = Ctchi::new(routes);
    ///     let server_result = match server.start() {
    ///         Ok(()) => "Ctchi application server is successfully running!".to_string(),
    ///         Err(err) => format!("Can't start server! Because '{}'", err)
    ///     };
    /// }
    /// ```
    pub fn start(self) -> std::io::Result<()> {

        let config_reader = get_configuration();
        let config = config_reader.inner.lock().unwrap();
        let bind_path= config.bind_path.to_string();
        let log_enabled = config.log_enabled;
        drop(config);

        if log_enabled {
            logger::init();
        }

        log::info!("Ctchi is running!");

        let listener = TcpListener::bind(bind_path)?;
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