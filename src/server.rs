use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
pub struct Server {
    port: String,
}

impl Server {
    pub fn default(port: &str) -> Self {
        Server {
            port: port.to_string(),
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let req_line = buf_reader.lines().next().unwrap().unwrap();

        let contents: String;
        let content_len: usize;

        let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "src/static/index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "src/static/404.html")
        };

        contents = fs::read_to_string(filename).unwrap();
        content_len = contents.len();

        let res = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
        stream.write_all(res.as_bytes()).unwrap();
    }

    pub fn run(&self) {
        let addr = ["127.0.0.1:", &self.port].join("");
        let listener = TcpListener::bind(addr).unwrap();
        println!("Serving at `:{}`", self.port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Incoming request!");
            Self::handle_connection(stream);
        }
    }
}
