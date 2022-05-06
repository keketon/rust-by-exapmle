use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use web_server::ThreadPool;

use std::fs::File;

use std::thread::sleep;
use std::time::Duration;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let mut pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let (status_line, file) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK\r\n\r\n", "assets/hello.html")
  } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
    sleep(Duration::new(5, 0));
    ("HTTP/1.1 200 OK\r\n\r\n", "assets/hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "assets/404.html")
  };

  let mut file = File::open(file).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();

  let response = format!("{}{}", status_line, contents);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
