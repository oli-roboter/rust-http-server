use std::str::FromStr;
use super::request::ParseError;

#[derive(Debug)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PATCH,
    PUT,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT,
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::GET),
            "POST" => Ok(Self::GET),
            "PATCH" => Ok(Self::GET),
            "PUT" => Ok(Self::GET),
            "OPTIONS" => Ok(Self::GET),
            "HEAD" => Ok(Self::GET),
            "TRACE" => Ok(Self::GET),
            "CONNECT" => Ok(Self::GET),
            _ => {
                return Err(ParseError::InvalidMethod);
            }
        }
    }
}

// pub struct MethodError;
