// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::{Write, Read}};

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
            //    let _ =handle_connection(_stream);
               let res = handle_connection404(_stream).unwrap();

               println!("ffff");

               
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_connection404(mut stream : TcpStream ) -> Result<(), std::io::Error> {

    let mut buf = String::new();
    print!("hello1");
    if let Ok(_) = stream.read_to_string(&mut buf){

        print!("hello12");

       let mut iter = buf.split("\r\n").into_iter();
       let line1=  iter.next();
       let line2 = iter.next();
       let line3 = iter.next();
       println!("kkkkkk Ah, good, I am able to read usize from serverline1 {:?}",line1);
       println!("kkkkkk Ah, good, I am able to read usize from serverline2 {:?}",line2);


        if let Some(line_as_str) = line1{
            let mut l1_as_wss = line_as_str.split_whitespace();
            let method = l1_as_wss.next();
            let path = l1_as_wss.next();
            // let proto = l1_as_wss.next();
            println!("kkkkkk path, pathpathpathpathpathpath {:?}",path);

           if let Some(path) = path {
            if  path == "/" {
               let err = stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).is_err();
               if err{
                panic!("kkkkkk error  HTTP/1.1 404 Not Found");
               }
               Ok(())
            } else {
                let err = stream.write(b"HTTP/1.1 200 OK\r\n\r\n".as_bytes()).is_err();
                if err{
                 panic!("kkkkkk  error HTTP/1.1 200 OK ");
                }  
                Ok(())
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
fn handle_connection(mut stream : TcpStream ) -> Result<(), std::io::Error> {



    // stream.write(b"HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;

    

    let response_str = "HTTP/1.1 200 OK\r\n\r\n";
    let result  =stream.write_all(response_str.as_bytes());
     handle_connection404(stream)

      
   
    
}