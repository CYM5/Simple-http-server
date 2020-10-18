use std::fmt::{Display, Formatter, Result as fmtResult};
#[derive(Copy, Clone, Debug)]
pub enum StatusCode{
    Ok=200,
    BadRequest=400,
    NotFound=404,
}

impl StatusCode{
    pub fn return_sentence(&self) -> &str{
        match self {
            Self::Ok => {return "Ok"},
            Self::BadRequest => {return "Bad Request"},
            Self::NotFound =>{return "Not Found"},
        }
    }
}
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{}", *self as u16)
    }
}
#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response{
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self{
        return Response{status_code,body};
    }
}

impl Display for Response{
    fn fmt(&self, f: &mut Formatter) -> fmtResult{
        let body = match &self.body{
            Some(b) => b,
            None => ""
        };
        //println!("{} , {}", self.status_code, self.status_code.return_sentence());
        write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.return_sentence(), body)
    }
}