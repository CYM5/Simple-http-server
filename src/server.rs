use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{request, response};
use std::convert::TryFrom;
use crate::http::request::Request;
use crate::http::response::{Response, StatusCode};
use std::fs;

pub struct Server{
    addr: String,
}

impl  Server {
    pub fn new(addr: String) -> Server{ //associated function =/= method
        //here we call it directly on struct, not related to it like method
        Server {
            addr //if the name of struct field == the name of the parameter you can replace addr: addr by just addr
        }
    }

    pub fn run(self){ //here the method run take the ownership of the struct, so at the end of run the struct will be disalocated
        //because self is a pointer to the all struct, to not take the ownership ==borrow pass &self
        println!("Server listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {  //using match to match all possible return cases, to perform error mitigation, match HAVE to cover all possible value
                                        // also _ is used as default matching for exemple _ => {}
                                        // Or we can ignore a return with this, for exemple Ok((stream, _)), with this the client_addr will not be saved
                Ok((mut stream, client_addr)) => {
                    println!("Connection from {}", client_addr);
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            //println!("Data received \n\n{}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) //Here I pass a slice cause I work with <T> type
                            {   Ok(request) => {
                                dbg!(&request);
                                let mut path = request.path;
                                //let mut contents;
                                //println!("{}", path);
                                path = match path{
                                    "/" => "index.html",
                                    _ => &path[1..],
                                };
                                //println!("path {}", &path);
                                let filename = "D:\\code\\http_server_single_thread\\src\\html\\".to_owned()+path;

                                println!("Serving {}", &filename);
                                let resp = match fs::read_to_string(filename){
                                    Ok(contents) => {
                                        let response = Response::new(StatusCode::Ok, Some(contents));
                                        response
                                    },
                                    Err(e) => {
                                        println!("Error serving file {}", e);
                                        let response = Response::new(StatusCode::NotFound, Some("<h1>NOT FOUND<h1>".to_string()));
                                        response
                                    },
                                };
                                //let mut response = Response::new(StatusCode::Ok, contents);
                                write!(stream, "{}", resp);
                            }
                                Err(e) => {println!("Failed to parse the request {}", e);
                                let response = Response::new(StatusCode::BadRequest, None);
                                write!(stream, "{}", response);}
                            }
                        }
                        Err(e)=> {println!("Error during reading data received {}", e);}
                    }
                }
                Err(e) => {
                    println!("Connection error : {}", e);
                }
            }
        }
    }
}