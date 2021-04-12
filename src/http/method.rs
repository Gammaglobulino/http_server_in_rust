use std::str::FromStr;
use super::ParseError;


#[derive(Debug)]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for HttpMethods {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),  
            _=> Err(MethodError),
        }
        
    }
}
pub struct MethodError;