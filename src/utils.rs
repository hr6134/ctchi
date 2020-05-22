#[macro_use]

#[macro_export]
macro_rules! render {
    ($x:tt) => {
        {
            use ctchi::core::config::get_configuration;
            let config_reader = get_configuration();
            let config = config_reader.inner.lock().unwrap();
            let page = format!("{}/{}", config.base_path, $x);
            let content = fs::read_to_string(page).unwrap_or_else(|error| { error.to_string() });
            content
        }
    }
}

#[macro_export]
macro_rules! routes {
    ($x:ident) => {
        concat_idents!(ctchi_routing_, $x)
    }
}