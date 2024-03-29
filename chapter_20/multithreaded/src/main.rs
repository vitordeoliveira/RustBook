use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use multithreaded::ThreadPool;

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //
    //     handle_connection(stream);
    // }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // this will eventually overwhelm the system because you’d be making new threads without any limit.
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("MAIN FINISH, so now lets start to drop the ThreadPool");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("Request: {:#?}", http_request);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();

    // match &request_line[..]
    let (status_line, filename) = match &http_request[0][..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}
