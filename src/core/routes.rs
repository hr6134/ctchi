use std::collections::HashMap;

pub struct Route {
    pub path: &'static str,
    pub render_action: fn(&str) -> String,
}

pub struct Routes {
    routes: HashMap<&'static str, Route>,
}

fn not_found(pref: &str) -> String {
    use std::fs;
    let content = fs::read_to_string(format!("{}/{}", pref, "404.html"))
        .unwrap_or_else(|error| { error.to_string() });
    content
}

impl Routes {
    pub fn new() -> Routes {
        Routes {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.insert(route.path, route);
    }

    pub fn get_route(&self, uri: &str) -> &Route {
        self.routes.get(uri).unwrap_or(&&Route {
            path: "/404",
            render_action: not_found
        })
    }
}
