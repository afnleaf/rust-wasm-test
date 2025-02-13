use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

//local
use server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "./public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./public/404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}\r\n\
            Content-Length: {length}\r\n\
            \r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("listener broke, oh no.");
    let request_pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        request_pool.execute(|| {
            handle_connection(stream);
        });

    }

    println!("Shutting down.");
}

