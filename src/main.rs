// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::{Write, Read}, time::Duration};

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
    print!("******************************************************************************************");
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
