use std::str::FromStr;
#[derive(Debug)]
pub enum Method{
    GET,
    POST,
    PUT,
    OPTIONS,
    DELETE,
    HEAD,
    CONNECT,
    TRACE,
    PATCH,
}

impl FromStr for Method{
    type Err = MethodError;
    fn from_str(s:&str) -> Result<Self, Self::Err>{
        match s {
            "GET" => Ok(Self::GET),
            "POST"  => Ok(Self::POST),
            "PUT"  => Ok(Self::PUT),
            "OPTIONS"  => Ok(Self::OPTIONS),
            "DELETE" => Ok(Self::DELETE),
            "HEAD"  => Ok(Self::HEAD),
            "CONNECT"  => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;