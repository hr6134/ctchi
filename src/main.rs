#![feature(concat_idents)]
mod utils;

use ctchi::core::app::Ctchi;
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::route;
use std::collections::HashMap;
use ctchi::templates::parser::Context;

#[route("/")]
fn index()-> String {
    let mut context = HashMap::<String, Context>::new();
    context.insert("test".to_string(), Context::BooleanValue(true));
    context.insert("my_name".to_string(), Context::SingleValue("Leonid Toshchev".to_string()));
    context.insert("numbers".to_string(), Context::MultiValue(vec!("1".to_string(), "2".to_string(), "3".to_string())));
    render!("index.html", context)
}

#[route("/blog/{id}/")]
fn blog(id: &str) -> String {
    let page = &format!("blog/{}.html", id);
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