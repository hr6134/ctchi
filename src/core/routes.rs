use regex::Regex;

pub struct Route {
    pub path: String,
    pub render_action: fn(&str) -> String,
}

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new() -> Routes {
        let mut routes = Routes {
            routes: Vec::new(),
        };

        routes.add_route(Route {
            path: "/404".to_string(),
            render_action: |_url| {
                "404 Not Found".to_string()
            },
        });

        routes
    }

    pub fn add_route(&mut self, route: Route) {
        let url_replacer = Regex::new(r"(?P<first>\{.+?\})").unwrap();
        let regex_url = url_replacer.replace_all(&route.path, ".+?");
        let string = format!(r"^{}/?$", regex_url.to_string());
        self.routes.push(Route {
            path: string,
            render_action: route.render_action,
        });
    }

    pub fn get_route(&self, uri: &str) -> &Route {
        for r in self.routes.iter() {
            let regex = Regex::new(&r.path).unwrap();
            if regex.is_match(uri) {
                return r
            }
        }

        self.get_route("/404")
    }
}
