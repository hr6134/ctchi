use std::collections::HashMap;

pub struct Routes<'a> {
    routes: HashMap<&'a str, &'a str>,
}

impl<'a> Routes<'a> {
    pub fn new() -> Routes<'a> {
        Routes {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, uri: &'a str, file: &'a str) {
        self.routes.insert(uri, file);
    }

    pub fn get_route(&self, uri: &'a str) -> &str {
        self.routes.get(uri).unwrap_or(&"/404")
    }
}
