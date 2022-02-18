// We bring std::io::prelude into scope to get access to certain traits that let us read from and write to the stream
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use multi_threaded_web_server::ThreadPool;

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // The incoming method on TcpListener returns an iterator that gives us a sequence of streams
    // A single stream represents an open connection between the client and the server.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // We’ve made the buffer 1024 bytes in size, which is big enough to hold the data of a basic request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        send_file_response(stream);
    } else if buffer.starts_with(sleep) {
        send_delayed_file_response(stream);
    } else {
        send_404_response(stream);
    }
}

fn send_basic_response(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    //  flush will wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();
}

fn send_file_response(mut stream: TcpStream) {
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_delayed_file_response(mut stream: TcpStream) {
    thread::sleep(Duration::from_secs(5));
    send_file_response(stream);
}

fn send_404_response(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}