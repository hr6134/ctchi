extern crate ctchi;

use ctchi::core::ctchi::{Ctchi, Config};

fn main() {
    let configuration = Config {
        bind_path: "127.0.0.1:8080",
        static_path: "/home/ltoshchev/programming/rust/ctchi/"
    };
    let server = Ctchi::new(configuration);
    server.start();
}