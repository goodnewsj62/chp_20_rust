
use std::{fs::read_to_string, io::{prelude::*,  BufReader}, net::{TcpListener, TcpStream}};

fn main() {
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream  in  listener.incoming() {
        let stream =  stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:  TcpStream){
    let reader =  BufReader::new(& mut stream);

    let http_request:Vec<_>  =  reader
                                    .lines()
                                    .map(|res| res.unwrap() )
                                    .take_while(|line| !line.is_empty())
                                    .collect();

    let response =  "HTTP/1.1 200 OK\r\n\r\n";

    handle_response(stream);

}

fn  handle_response(mut stream:  TcpStream){
    let resp  =  String::from("HTTP/1.1 200 OK\r\n");
    let message =  read_to_string("/home/routebirds/Documents/work_folder/chp_20_rust/src/hello.html").unwrap();
    let length =  message.len();

    let resp =  format!("{resp}Content-Length: {length}\r\n\r\n{message}");
    stream.write_all(resp.as_bytes()).unwrap();
}
