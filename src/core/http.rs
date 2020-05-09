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