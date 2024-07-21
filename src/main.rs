
use std::{fs::read_to_string,thread,  time::Duration, io::{prelude::*,  BufReader}, net::{TcpListener, TcpStream}, path::Path};
use chp_20_rust::ThreadPool;


fn main() {
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool =  ThreadPool::new(4);

    for stream  in  listener.incoming() {
        let stream =  stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream:  TcpStream){
    let reader =  BufReader::new(& mut stream);

    

    let request_line =  reader.lines().next().unwrap().unwrap();
    let   header; let  path;

    match &request_line[..] {
        "GET / HTTP/1.1" => {
            (header, path) =  ("HTTP/1.1 200 OK",  "hello.html");
        },
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (header, path) =  ("HTTP/1.1 200 OK",  "hello.html");
        },
        _ =>{
            (header, path) =  ("HTTP/1.1 404 NOT FOUND",  "404.html");
        }
    }


    handle_response(stream, &header, &path);
}

fn  handle_response<T>(mut stream:  TcpStream, header:  &str, path:T )
where T: AsRef<Path>
{
    let message =  read_to_string(path).unwrap();
    let length =  message.len();

    let resp =  format!("{header}\r\nContent-Length: {length}\r\n\r\n{message}");
    stream.write_all(resp.as_bytes()).unwrap();
}
