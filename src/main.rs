// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::{Write, Read}, time::Duration, result, ops::Deref};

 use itertools::Itertools;
use nom::AsBytes;
 



fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();


    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection1");
                
                let res = handle_connection404(_stream);

                println!("{:?}",res);


               
            }
            Err(e) => {
                println!("errorerrorerrorerrorerrorerrorerror: {}", e);
            }
        }
    }
}


fn handle_connection404(mut stream : TcpStream ) {

    let mut buf_bytes = [0;2048];
    let _ = stream.set_read_timeout(Some(Duration::from_secs(20)));
    println!("*********************************************************");
    if let Ok(_) = stream.read(&mut buf_bytes){

        let mut buf = String::from_utf8(buf_bytes.into()).ok().unwrap();
        let mut iter = buf.split("\r\n");
        let line1=  iter.next();
        let _ = iter.next();
        let _ = iter.next();
         if let Some(line_as_str) = line1{
             let mut l1_as_wss = line_as_str.split_whitespace();
             let _ = l1_as_wss.next();
             let path = l1_as_wss.next();
 
            if let Some(path) = path {
                println!("kkkkkk path, pathpathpathpathpathpath {:?}",path);
             if  path == "/" {
                eprintln!("path is 200");
                let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
           
             } else  if  path.contains("/echo/") {
                eprintln!("path contains -------/ and 200");
                let results = path.split("/").filter(|p|!p.is_empty()).collect_vec();
                eprintln!("XXxXX {results:?}");
    let _ = stream.write(b"HTTP/1.1 200 OK\r\n");
    let _ = stream.write(b"Content-Type: text/plain\r\n");
    let res = results.last().unwrap_or(&"");
    let echo_parts: Vec<_> = results.iter().skip(1).cloned().collect();
    let res2 = echo_parts.join("/");
    let content_length = res2.len() as i32;
    let content_length_header = format!("Content-Length: {}\r\n", content_length);
    let content_length_header = content_length_header.as_str();
    let _ = stream.write(content_length_header.as_bytes());
    // End of headers
    let _ = stream.write(b"\r\n");
    let _ = stream.write(res2.as_bytes());

             } else {
                eprintln!("path is 400");
                let _ = stream.write(b"HTTP/1.1 404 OK\r\n\r\n");
 
             }
 
            } else {
             panic!("kkkkkk 444 error HTTP/1.1 200 OK ");
            }
         } else {
             panic!("kkkkkk 555 error HTTP/1.1 200 OK ");
         }
        
     } else {
         panic!("kkkkkk  can't read data from server ")
     }

   

}


fn split_string_by_slash( input : &str) -> Option<Vec<&str>> {


    let mut result = Vec::new();
    let mut start = 0;
    let mut end = 0 ;

    for (idx, cha) in input.chars().enumerate(){

        eprint!("split_string_by_slash {:?}", cha);
        if(cha == '/'){
            end = idx;
            if (end - start) !=0{
                result.push(&input[start..end]);
                start = idx+1;
            }
        }



    }
    result.push(&input[start..]);

    Some(result)

}