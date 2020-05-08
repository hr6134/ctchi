extern crate ctchi;

use ctchi::core::ctchi::{Ctchi, Config, Routes};

fn main() {
    let mut routes = Routes::new();
    routes.add_route("/", "/src/static/index.html");

    let configuration = Config {
        bind_path: "127.0.0.1:8080",
        static_path: "/home/ltoshchev/programming/rust/ctchi/",
        routes,
    };

    let server = Ctchi::new(configuration);
    server.start();
}