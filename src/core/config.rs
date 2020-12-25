use std::env::current_dir;
use std::fs;
use std::sync::{Arc, Mutex, Once};
use core::mem;

#[derive(Debug)]
pub struct Config {
    pub bind_path: String,
    pub base_path: String,
    pub static_uri_pref: String,
    pub log_path: String,
}

impl Config {
    pub fn new() -> Config {
        Config::parse_config("/etc/ctchi/conf.txt")
    }

    fn parse_config(path: &str) -> Config {
        let mut bind_path = "127.0.0.1:8080";
        let mut log_path = "/Users/glotitude/log/ctchi/server.log";

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
            if cols[0] == "log_path" {
                log_path = cols[1];
            }
        }

        Config {
            bind_path: bind_path.to_string(),
            base_path: templates_dir,
            static_uri_pref: "/static/".to_string(),
            log_path: log_path.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ConfigReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    pub inner: Arc<Mutex<Config>>,
}

pub fn get_configuration() -> ConfigReader {
    // Initialize it to a null value
    static mut SINGLETON: *const ConfigReader = 0 as *const ConfigReader;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = ConfigReader {
                inner: Arc::new(Mutex::new(Config::new())),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}