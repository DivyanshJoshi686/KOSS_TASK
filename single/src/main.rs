use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use single::ThreadPool;
use std::sync::{Arc,Mutex};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| 
            {handle_connection(stream)});
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_lines: Vec<String> = buf_reader.lines()
        .map(|line| line.unwrap_or_default())
        .take_while(|line| !line.is_empty()) 
        .collect();

    if request_lines.is_empty() {
        return;
    }

    let request_line = &request_lines[0]; 
    println!("Received request: {}", request_line); 

    let response = match request_lines[0].trim() {
        "GET / HTTP/1.1" => response_text("Click on buttons to get responses"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            response_text("Responded after delay")
        },
        "GET /name HTTP/1.1" => response_text("Divyansh Joshi"),
        "GET /branch HTTP/1.1" => response_text("Computer Science and Engineering"),
        "GET /college HTTP/1.1" => response_text("IIT Kharagpur"),
        "GET /involvements HTTP/1.1" => response_text("Communique"),
        "GET /department HTTP/1.1" => response_text("Computer Science and Engineering"),
        _ => response_text("404 NOT FOUND"),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn response_text(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: application/json\r\n\
        Access-Control-Allow-Origin: *\r\n\
        \r\n\
        \"{}\"",
        body.len() + 2,
        body
    )
}