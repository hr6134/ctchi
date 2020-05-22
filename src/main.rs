#![feature(concat_idents)]
mod utils;

use ctchi::core::app::Ctchi;
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::route;

use std::fs;

#[route("/")]
fn index()-> String {
    render!("index.html")
}

#[route("/blog/{id}/")]
fn blog(id: &str) -> String {
    let page = format!("blog/{}.html", id);
    render!(page)
}

fn main() {
    let mut routes = Routes::new();
    routes.add_route(routes!(index)());
    routes.add_route(routes!(blog)());

    let server = Ctchi::new(routes);
    let server_result = match server.start() {
        Ok(()) => "Ctchi application server is successfully running!".to_string(),
        Err(err) => format!("Can't start server! Because '{}'", err)
    };

    println!("{}", server_result);
}