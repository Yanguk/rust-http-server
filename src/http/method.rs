use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    Get,
    Delete,
    Post,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "DELETE" => Ok(Self::Delete),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
