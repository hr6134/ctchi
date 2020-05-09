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

pub struct Request {
    pub method: HttpMethod,
    pub url: String,
    pub headers: String,
    pub body: String,
}