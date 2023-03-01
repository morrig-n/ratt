pub mod parser;

use std::collections::HashMap;
use std::net::{TcpListener, Shutdown, TcpStream};
use std::io::{Read, Write};

pub struct App {
    registered_routes: HashMap<String, HashMap<HTTP, RegisteredRoute>>
}

pub struct RegisteredRoute {
    text: String    
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum HTTP {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS
}

pub enum HTTPVersion {
    One,
    OnePointOne,
    Two
}

impl App {
    pub fn new() -> Self {
        App {
            registered_routes: HashMap::<String, HashMap<HTTP, RegisteredRoute>>::new()
        }
    }

    pub fn register<T>(&mut self, path: &str, method: HTTP, mut callback: T) where T: FnMut(i8, i8) -> String { 
        let currently_registered = self.registered_routes.get_mut(path);

        match currently_registered {
            None => {
                let mut map = HashMap::<HTTP, RegisteredRoute>::new();
                map.insert(method, RegisteredRoute { text: callback(0, 0) });
                self.registered_routes.insert(path.to_string(), map);
            },
            Some(map) => {
                let already_exists = map.get(&method).is_some();
                if already_exists {
                    eprintln!("ERROR: Registered the same route ({method:?} {path}) twice.");
                } else {
                    map.insert(method, RegisteredRoute { text: callback(0, 0) });
                }
            }
        }
    }

    fn send_status(&self, stream: &mut TcpStream, status: usize) -> std::io::Result<()> {
        let meta = match status {
            404 => "404 Not Found",
            405 => "405 Method Not Allowed",
            _ => "200 OK"
        }.as_bytes();

        stream.write(b"HTTP/1.1 ")?;
        stream.write(meta)?;
        stream.write(b"\r\n\r\n")?;

        stream.flush()?;
        stream.shutdown(Shutdown::Both)?;

        Ok(()) 
    }

    pub fn listen(&mut self, port: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1{}", port))?;

        for stream in listener.incoming() {
            match stream {
                Err(e) => eprintln!("ERROR: {e}"),
                Ok(mut s) => {
                    // TODO: Handle concurrent connections somehow?
                    // Multi-threading maybe?

                    let mut buffer = [0; 1024];
                    let mut message = String::new();
                    loop {
                        // Some notes:
                        // We read until \r\n\r\n **if no body**
                        // Body present = required Content-Length header
                        // Therefore we must check for Content-Length header, then decide whether
                        // to keep reading. If we do, then we must read exactly the Content-Length
                        // amount of bytes as per specification.
                        //
                        // TODO: ^ this (i.e. body reading)

                        let byte_count = s.read(&mut buffer)?;

                        if byte_count == 0 {
                            break;
                        }
                    
                        message.push_str(std::str::from_utf8(&buffer).unwrap());

                        if message.contains("\r\n\r\n") {
                            break;
                        }    
                    }
                    
                    println!("{message}");

                    let meta = parser::parse_http_meta(&message).unwrap();

                    let router = self.registered_routes.get_mut(&meta.path);

                    if router.is_none() {
                        self.send_status(&mut s, 404)?;
                        continue;
                    }

                    let msg = router.unwrap().get(&meta.method);

                    if msg.is_none() {
                        self.send_status(&mut s, 405)?;
                        continue;
                    }

                    s.write(b"HTTP/1.1 200 OK\r\nConnection: keep-alive\r\nContent-Type: application/json; charset=utf-8\r\nKeep-Alive: timeout=5\r\n\r\n{\"success\":\"")?;
    
                    // Manually route as GET /
                    s.write(msg.unwrap().text.as_str().as_bytes())?;
                    s.write(b"\"}")?;

                    s.flush()?;
                }
            }
        }

        Ok(())
    }
}
