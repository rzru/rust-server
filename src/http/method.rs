use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::GET),
            "PUT" => Ok(Method::GET),
            "PATCH" => Ok(Method::GET),
            "DELETE" => Ok(Method::GET),
            "HEAD" => Ok(Method::GET),
            "CONNECT" => Ok(Method::GET),
            "OPTIONS" => Ok(Method::GET),
            "TRACE" => Ok(Method::GET),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;