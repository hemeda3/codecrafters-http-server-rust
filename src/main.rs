// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();


    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                let _ = handle_connection(_stream);
                // _stream.w
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream : TcpStream ) ->  Result<(), std::io::Error> {

    let response_str = "HTTP/1.1 200 OK\r\n\r\n";
     let result  =stream.write_all(response_str.as_bytes());

     result

    
}