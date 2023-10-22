// Uncomment this block to pass the first stage
use std::env;


use tokio::{net::{TcpListener,TcpStream}, io::{AsyncReadExt, AsyncWriteExt}, fs::File};


#[tokio::main]
async fn main() ->Result<(), anyhow::Error>{
    
    let listener = TcpListener::bind("127.0.0.1:4221").await?;
    while let(mut stream) = listener.accept().await? {
        let dir_args: Option<String> = env::args().nth(2);

        // Spawn a new Tokio task to handle each incoming connection
        println!("Spawn a new Tokio task to handle each incoming connection");
        tokio::spawn(async move {
            if let Err(e) = handle_connection404(stream.0,dir_args.clone()).await {
                // Log the error here
                eprintln!("Error while errrrr handling connection: {:?}", e);
            }
        });
    }
    Ok(())
    

}

pub struct HttpRequest {

}

pub struct  HttpResposne {

}

async fn handle_connection404(mut stream: TcpStream, director : Option<String>) -> Result<(), anyhow::Error> {
    let mut buf_bytes = [0; 2048];
    println!("********************************************************* {:?}",director);
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
                println!("@@Genral path {:?}", path);
                if path == "/" {
                    eprintln!("Path is 200");
                    let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\n").await?;
                    Ok(())
                } else if path.contains("/echo/") {
                    eprintln!("Path is /echo/");
                    let results = path.split("/").filter(|p| !p.is_empty()).collect::<Vec<_>>();
                    eprintln!("XXxXX {results:?}");
                    let _ = stream.write(b"HTTP/1.1 200 OK\r\n").await?;
                    let _ = stream.write(b"Content-Type: text/plain\r\n").await?;
                    let res = results.last().unwrap_or(&"");
                    let echo_parts: Vec<_> = results.iter().skip(1).cloned().collect();
                    let res2 = echo_parts.join("/");
                    let content_length = res2.len() as i32;
                    let content_length_header = format!("Content-Length: {}\r\n", content_length);
                    let content_length_header = content_length_header.as_str();

                    let _ = stream.write(content_length_header.as_bytes()).await?;
                    // End of headers
                    let _ = stream.write(b"\r\n").await?;
                    let _ = stream.write(res2.as_bytes()).await?;
                    Ok(())
                } else if path.contains("/user-agent") {
                    eprintln!("Path is /user-agent");
                    let len = user_agent.map_or(0, |s| {
                        s.len()
                    });
                    let _ = stream.write(b"HTTP/1.1 200 OK\r\n").await?;
                    let _ = stream.write(b"Content-Type: text/plain\r\n").await?;
                    let len_msg = format!("Content-Length: {}\r\n",len);
                    let _ = stream.write(len_msg.as_bytes()).await?;
                    let _ = stream.write(b"\r\n").await?;
                    let _ = stream.write(format!("{}",user_agent.unwrap_or("")).as_bytes()).await?;
                    Ok(())
                } else if path.contains("/files/") {
                    eprintln!("Path is /files/");

                    if let Some(file_name) =  path.strip_prefix("/files/") {
                        let full_path = format!("{}/{}",director.clone().unwrap_or("".to_string()),file_name);
                        println!("ffffffffffffffffffffffff {:?} {:?} ", director,file_name);

                        let mut file = File::open(full_path).await;

                        match file {
                            Ok(mut file) =>{
                                eprintln!("*** yes file  found 200 ");

                                let mut buf = String::new();
                                file.read_to_string(&mut buf).await?;
                                println!("ffffffffffffffffffffffff {} ",buf);
                                let _ = stream.write(b"HTTP/1.1 200 OK\r\n").await?;
                                let _ = stream.write(b"Content-Type: application/octet-stream\r\n").await?;
                                let len_msg = format!("Content-Length: {}\r\n",buf.len());
                                let _ = stream.write(len_msg.as_bytes()).await?;
                                let _ = stream.write(b"\r\n").await?;
                                let _ = stream.write(buf.as_bytes()).await?;
                              
                            },
                            Err(e) =>{
                                eprintln!("***file not found 404 ");
                                let _ = stream.write(b"HTTP/1.1 404 OK\r\n\r\n").await?;
                            }
                        }
                      
                    }
                    Ok(())
                } else{
                    eprintln!("####last case path is 400");
                    let _ = stream.write(b"HTTP/1.1 404 OK\r\n\r\n").await?;

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
