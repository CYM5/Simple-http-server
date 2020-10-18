#![allow(dead_code)]
#![allow(non_snake_case)]
use server::Server;
mod server;
mod http;
fn main() {
    let Server = Server::new(String::from("127.0.0.1:1234"));
    //    let Server = Server::new("127.0.0.1:1234".to_string()); //this is needed because "127.....4" is not a string
    //but immutable ref to the string, because the string is knowed at compiled time so the string is stored in memory
    Server.run();
}



