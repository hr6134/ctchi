#![feature(concat_idents)]
mod utils;

use ctchi::core::app::{Ctchi, Config};
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::route;

use std::fs;

#[route("/")]
fn index()-> String {
    render!("/home/ltoshchev/programming/rust/ctchi/src/pages/index.html")
}

#[route("/blog/{id}/")]
fn blog(id: &str) -> String {
    let page = format!("/home/ltoshchev/programming/rust/ctchi/src/pages/blog/{}.html", id);
    render!(page)
}

fn main() {
    let mut routes = Routes::new();
    routes.add_route(routes!(index)());
    routes.add_route(routes!(blog)());

    let config = Config::new();

    println!("{:?}", config);

    let server = Ctchi::new(config, routes);
    let server_result = match server.start() {
        Ok(()) => "Ctchi application server is successfully running!".to_string(),
        Err(err) => format!("Can't start server! Because '{}'", err)
    };

    println!("{}", server_result);
}