#[macro_use]

#[macro_export]
macro_rules! render {
    ($x:tt) => {
        {
            let content = fs::read_to_string($x).unwrap_or_else(|error| { error.to_string() });
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