extern crate ctchi;

use ctchi::core::ctchi::{Ctchi, Config};
use ctchi::core::routes::Routes;

fn main() {
    let mut routes = Routes::new();
    routes.add_route("/", "/static/index.html");
    routes.add_route("/favicon.ico", "/static/index.html");

    let configuration = Config {
        bind_path: "127.0.0.1:8080",
        base_path: "/home/ltoshchev/programming/rust/ctchi/src/",
        static_uri_pref: "/static/",
        routes,
    };

    let server = Ctchi::new(configuration);
    server.start();
}