##Home made web framework written in Rust
What you are looking at is my homemade web framework. It started as pet project 
and probably continue to be one, but now it has enough features to start simple 
web page, it even has templates. 

Is it ready for production? Not in any case. It doesn't have escaping. It doesn't parse input headers. 
No middleware. List could go on and on. 

But, I have personal page that runs on this framework at http://glotitude.datamonkey.pro/. Feel free to look at it.

###Fast start
* You should use nightly build because of `concat_idents!` macro
* Add dependencies to ctchi framework
```
[dependencies]
ctchi = "0.13.0"
ctchi_codegen = "0.2.0"
regex = "1"
```
* Write your controller and main function. You need all imports below
```rust
#![feature(concat_idents)]
#[macro_use]
extern crate ctchi;

use ctchi::core::app::Ctchi;
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::route;

#[route("/")]
fn index() -> String {
    render!("index.html")
}

fn main() {
    let mut routes = Routes::new();
    // add route to your controller
    routes.add_route(routes!(index)());

    // create and run local server
    let server = Ctchi::new(routes);
    let server_result = match server.start() {
        Ok(()) => "Ctchi application server is successfully running!".to_string(),
        Err(err) => format!("Can't start server! Because '{}'", err)
    };
}
```
* Run it :)
```shell script
cargo run
```

###Configuration
Ctchi has just a few configuration options:
1. `bind_path` - ip address and port for the server (default is `127.0.0.1:8080`)
2. `base_path` - path to the folder with templates (default is current_dir + `/src/pages/`)
3. `static_uri_pref` - url prefix for static files, css/js/images etc (default is `static`)


There are several ways to change configuration of the server:
####Change configuration file
You should create it at `/etc/ctchi/conf.txt`. You can overwrite only 2 
properties there `bind_path` and `base_path`.
####Configuration singleton
You can get ctchi configuration in any place of your program by importing 
`use crate::core::config::get_configuration;`
Next you can get a reader and then mutex for configuration:
```rust
// get reader
let config_reader = get_configuration();
// get mutex
let config = config_reader.inner.lock().unwrap();
// read some propeties
let prefix = &config.static_uri_pref;
// drop mutex
drop(config);
```

`drop` is very important. You can omit it and it will be automatically called 
at the end of the scope, but, if you try to get another config inside the 
scope program wouldn't work. So, if you absolutely sure that there wouldn't 
be second call for configuration you don't need `drop`, if not, better to 
call it explicitly.

###Template
Ctchi has html templates engine. It isn't sophisticated, but it has all core features you need.
What kind of tags it has.
1. [templates][endtemplates]
2. [for i in values][endfor]
3. [if value][endif]
4. [import "./base.html" /]
5. [[value]]

`[templates]` is root tag, if you have it on the page it is html page with tags, 
otherwise ctchi would consider it plane html page.

`[for]` tag is for loops. You can pass values (vector of strings) in context and it write 
inner part so many time as values length.

`[if]` takes boolean value from context and writes inner block if values is true. 
It hasn't `else` clause. Is can't take expression. So, the whole logic should be 
on backend.

`[import]` gets page from specified page and import it into current template. 
Every rule about tags applies to that page as well.

`[[value]]`. Plain value or variable of for loop should be taken in double square brackets.

Every tag except import and value tags, should has closed part.

####Example
Remember, all html pages should be in `src/pages` folder by default. 
Now, say, we have same header for every page. Let's put it into `header.html`:
```html
<head>
    <meta charset="UTF-8">
    <title>Imported Header</title>
    <link href="/static/css/main.css" type="text/css" rel="stylesheet">
</head>
```  

And we have index.html:
```html
[template]
<html lang="en">
    [import "header.html"/]
<body>
[if authorized]
    Hello [[user_name]].
[endif]

Your options for today are:
<ul>
[for option in options]
    <li>[[option]]</li>
[endfor]
</ul>
</body>
</html>
[endtemplate]
```

So, how  our controller would look for such template:
```rust
#[route("/")]
fn index()-> String {
    let mut context = HashMap::<String, Context>::new();
    context.insert("authorized".to_string(), Context::BooleanValue(true));
    context.insert("user_name".to_string(), Context::SingleValue("Leonid Toshchev".to_string()));
    context.insert("options".to_string(), Context::MultiValue(vec!("Eat".to_string(), "Code".to_string(), "Sleep".to_string())));
    render!("index.html", context)
}
```