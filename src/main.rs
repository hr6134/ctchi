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
    context.insert("names".to_string(), Context::MultiValue(vec!("Leonid".to_string(), "Daria".to_string(), "Ilya".to_string())));
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

    let configuration = Config {
        bind_path: "127.0.0.1:8080",
        base_path: "/home/ltoshchev/programming/rust/ctchi/src/static/",
        static_uri_pref: "/css/",
    };

    Config::new();

    let server = Ctchi::new(configuration, routes);
    server.start();
}