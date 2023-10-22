// Uncomment this block to pass the first stage
use std::{
    ops::Deref,
    result,
    time::Duration,
};

// use itertools::Itertools;
use nom::AsBytes;
use tokio::{net::{TcpListener,TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};


#[tokio::main]
async fn main() ->Result<(), anyhow::Error>{
    
    let listener = TcpListener::bind("127.0.0.1:4221").await?;
    while let Ok(mut stream) = listener.accept().await {
        // Spawn a new Tokio task to handle each incoming connection
        tokio::spawn(handle_connection404(stream.0));

    }
    Ok(())

}

pub struct HttpRequest {

}

pub struct  HttpResposne {

}

async fn handle_connection404(mut stream: TcpStream) {
    let mut buf_bytes = [0; 2048];
    println!("*********************************************************");
    if let Ok(_) = stream.read(&mut buf_bytes).await {
        let mut buf = String::from_utf8(buf_bytes.clone().into()).ok().unwrap();
        let mut body_buff = String::from_utf8(buf_bytes.clone().into()).ok();
        let(headers,body) =  body_buff.map_or((None,None),|s| {
            let parts = s.split("\r\n\r\n").collect::<Vec<_>>();
           match parts.len() {
            0 => (None,None),
            1 => (Some(parts[0].to_string().split("\r\n").map(String::from).collect::<Vec<_>>()),None),
            _ => (Some(parts[0].to_string().split("\r\n").map(String::from).collect::<Vec<_>>()),Some(parts[1].to_string()))
               
           }
        });

        let user_agent = headers.as_ref().and_then(|v| {
            v.iter().find(|s| s.starts_with("User-Agent: ")).map_or(None,|s|s.strip_prefix("User-Agent: "))
        });
        println!("here all headers == {:?}",headers);
        // println!("Here body extracted == {:?}",body);
        println!("Extracted user agent user_agent == {:?}",user_agent);


        let mut iter = buf.split("\r\n");
        let mut body :Vec<&str> = iter.clone().collect::<Vec<_>>();

        let line1 = iter.next();
        let line2_first_header = iter.next();
        let line3_2nd_header = iter.next();


        while let Some((idx,line) )= body.iter().enumerate().next() {
            if *line =="\r\n"{
               
                break;;
            }
            if body.len()>0 {
                body.remove(idx);

            }
            
        }

        println!("bodybodybodybodybody == {:?}",body);

        if let Some(line_as_str) = line1 {
            let mut l1_as_wss = line_as_str.split_whitespace();
            let _ = l1_as_wss.next();
            let path = l1_as_wss.next();

            if let Some(path) = path {
                println!("kkkkkk path, pathpathpathpathpathpath {:?}", path);
                if path == "/" {
                    eprintln!("path is 200");
                    let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                } else if path.contains("/echo/") {
                    eprintln!("path contains -------/ and 200");
                    let results = path.split("/").filter(|p| !p.is_empty()).collect::<Vec<_>>();
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
                } else if path.contains("/user-agent") {
                    eprintln!(" user agent path ");
                    let len = user_agent.map_or(0, |s| {
                        s.len()
                    });
                    let _ = stream.write(b"HTTP/1.1 200 OK\r\n");
                    let _ = stream.write(b"Content-Type: text/plain\r\n");
                    let len_msg = format!("Content-Length: {}\r\n",len);
                    let _ = stream.write(len_msg.as_bytes());
                    let _ = stream.write(b"\r\n");
                    let _ = stream.write(format!("{}",user_agent.unwrap_or("")).as_bytes());

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

fn split_string_by_slash(input: &str) -> Option<Vec<&str>> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut end = 0;

    for (idx, cha) in input.chars().enumerate() {
        eprint!("split_string_by_slash {:?}", cha);
        if (cha == '/') {
            end = idx;
            if (end - start) != 0 {
                result.push(&input[start..end]);
                start = idx + 1;
            }
        }
    }
    result.push(&input[start..]);

    Some(result)
}
