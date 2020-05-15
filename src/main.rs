extern crate ctchi;

use ctchi::core::app::{Ctchi, Config};
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::static_page;


#[static_page("/", "index.html")]
fn index()-> Route {}

fn main() {
    let mut routes = Routes::new();
    routes.add_route(index());

    let mut config = Config::new();

    let server = Ctchi::new(config, routes);
    let server_result = match server.start() {
        Ok(()) => "Ctchi application server is successfully running!",
        Err(_) => "Can't start server!"
    };

    println!("{}", server_result);
}