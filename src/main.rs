extern crate ctchi;

use ctchi::core::app::{Ctchi, Config};
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::static_page;


#[static_page("/", "index.html")]
fn index()-> Route {}

fn main() {
    let mut routes = Routes::new();
    routes.add_route(index());

    let configuration = Config {
        bind_path: "127.0.0.1:8080",
        base_path: "/home/ltoshchev/programming/rust/ctchi/src/static/",
        static_uri_pref: "/css/",
    };

    Config::new();

    let server = Ctchi::new(configuration, routes);
    server.start();
}