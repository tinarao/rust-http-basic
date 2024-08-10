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
        let _req_data: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("src/index.html").unwrap();
        let length = contents.len();

        let res = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

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
