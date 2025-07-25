use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
-- Test 
-- Test
fn main() {
    let listener = TcpListener::bind("brainiac.attlocal.net:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
