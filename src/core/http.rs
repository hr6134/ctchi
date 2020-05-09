/// Represent HTTP method + unknown value in case we are missing something in enum or
/// client send us wrong request.
#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
    PATCH,
    UNKNOWN,
}

impl HttpMethod {
    /// Build HttpMethod enum from string. If it is valid HTTP method we get appropriate value.
    /// Otherwise we get HttpMethod::UNKNOWN
    ///
    /// # Arguments:
    /// * `method_str` - string which contains HTTP method representation
    ///
    /// # Example
    ///
    /// ```rust
    /// use ctchi::core::http::HttpMethod;
    /// assert!(HttpMethod::parse("GET") == HttpMethod::GET);
    /// assert!(HttpMethod::parse("POST") == HttpMethod::POST);
    /// assert!(HttpMethod::parse("PUT") == HttpMethod::PUT);
    /// assert!(HttpMethod::parse("SOMETHING") == HttpMethod::UNKNOWN);
    /// ```
    pub fn parse(method_str: &str) -> HttpMethod {
        match method_str {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD" => HttpMethod::HEAD,
            "CONNECT" => HttpMethod::CONNECT,
            "TRACE" => HttpMethod::TRACE,
            "PATCH" => HttpMethod::PATCH,
            _ => HttpMethod::UNKNOWN,
        }
    }
}

pub struct Request {
    pub method: HttpMethod,
    pub url: String,
    pub headers: String,
    pub body: String,
}