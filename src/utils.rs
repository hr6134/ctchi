#[macro_export]
macro_rules! render {
    ($x:tt) => {
        {
            use ctchi::templates::parser;
            use ctchi::templates::writer;

            let tag = parser::parse_file($x);
            let context = HashMap::new();
            let result = writer::write(&tag, &context);
            result
        }
    };
    ($x:tt, $c:ident) => {
        {
            use ctchi::templates::parser;
            use ctchi::templates::writer;

            let tag = parser::parse_file($x);
            let result = writer::write(&tag, &$c);
            result
        }
    }
}

#[macro_export]
macro_rules! routes {
    ($x:ident) => {
        concat_idents!(ctchi_routing_, $x)
    }
}