use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Debug, Result as fmtResult};
use std::str;
use super::query_string;
use crate::http::query_string::QueryString;
#[derive(Debug)]
pub struct Request<'buf>{
    method: Method,
    pub path: &'buf str,
    query_path: Option<QueryString<'buf>>,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;
    fn try_from(buf : &'buf[u8]) -> Result<Request<'buf>, Self::Error>{
        //println!("BUFFER RECEIVED HERE {}", String::from_utf8_lossy(&buf));
        //println!("END");
        let request = str::from_utf8(&buf).or(Err(ParseError::InvalidEncoding))?;
        //println!("REQUEST RECEIVED HERE {}", request);
        //println!("END");
        //Because from_utf8 return a Result I have to handle the error
        //this can replace the match pattern used before
        //to propagate the error
        let (method, request) = get_word_in_request(&request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_word_in_request(&request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_word_in_request(&request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            //println!("{}",protocol);
            return Err(ParseError::InvalidProtocol);
        }
        let method : Method = method.parse().or(Err(ParseError::InvalidMethod))?;
        let mut query_path = None;
        let q=path.find('?');
        if q.is_some(){
            let i = q.unwrap();
            query_path = Some(QueryString::from(&path[i + 1 ..]));
            //println!("QUERY PATH");
            //dbg!(query_path);
            path = &path[..i];
        }
        Ok(Self {
            path,
            query_path,
            method
        })
    }
}
fn get_word_in_request(req: &str) -> Option<(&str, &str)>{ //The return can be empty so we have to return an option to be able to return None
    for (i,c) in req.chars().enumerate(){
        if c== ' ' || c == '\r' {
            return Some((&req[..i], &req[i + 1..])); //addind in index is super dangerous because we add byte, not char, so in case of char code with more than 1 byte we can crash the program
                                                    //But here come after finding a space so it can't be another char, so no crash possible here
        }
    }
    return None;
}
pub enum ParseError{
    InvalidRequest,
    InvalidMethod,
    InvalidProtocol,
    InvalidEncoding,
}

impl ParseError {
    fn message(&self) -> &str{
        match self {
            Self::InvalidEncoding => { return "Invalid Encoding"; }
            Self::InvalidRequest => { return "Invalid Request"; }
            Self::InvalidMethod => { return "Invalid Method"; }
            Self::InvalidProtocol => { return "Invalid Protocol"; }
        }
    }
}
impl Display for ParseError {
     fn fmt(&self, f: &mut Formatter) -> fmtResult {
         write!(f, "{}", self.message())
         }
 }

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError{}