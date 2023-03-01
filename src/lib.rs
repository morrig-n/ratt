pub mod parser;

use std::collections::HashMap;
use std::net::{TcpListener, Shutdown, TcpStream};
use std::io::{Read, Write};

pub struct App {
    registered_routes: HashMap<String, HashMap<HTTP, RegisteredRoute>>
}

pub struct RegisteredRoute {
    callback: Box<dyn FnMut(i8, Response) -> Response>
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

pub struct Response {
    // TODO: Maybe move this to an Enum?
    status: usize,
    body: String
}

impl Response {
    pub fn set_status(&self, status: usize) -> Self {
        Response {
            status,
            body: self.body.clone()
        }
    }

    pub fn send(&self, content: String) -> Self {
        Response {
            status: self.status,
            body: content
        }
    }
}

fn send_response_object(stream: &mut TcpStream, response: Response) -> std::io::Result<()> {
    stream.write(b"HTTP/1.1 ")?; 
    stream.write(match response.status {
        200 => b"200 OK",
        201 => b"201 Created",
        400 => b"400 Bad Request",
        404 => b"404 Not Found",
        500 => b"500 Internal Server Error",
        _ => b"200 OK"
    })?;

    stream.write(b"\r\nConnection: keep-alive\r\nContent-Type: text/plain; charset=utf-8\r\nKeep-Alive: timeout=5\r\n\r\n")?;
    stream.write(response.body.as_bytes())?;

    Ok(())
}

impl App {
    pub fn new() -> Self {
        App {
            registered_routes: HashMap::<String, HashMap<HTTP, RegisteredRoute>>::new()
        }
    }

    pub fn register<T>(&mut self, path: &str, method: HTTP, callback: T) where T: FnMut(i8, Response) -> Response + 'static { 
        let currently_registered = self.registered_routes.get_mut(path);

        match currently_registered {
            None => {
                let mut map = HashMap::<HTTP, RegisteredRoute>::new();
                map.insert(method, RegisteredRoute { callback: Box::from(callback) });
                self.registered_routes.insert(path.to_string(), map);
            },
            Some(map) => {
                let already_exists = map.get(&method).is_some();
                if already_exists {
                    eprintln!("ERROR: Registered the same route ({method:?} {path}) twice.");
                } else {
                    map.insert(method, RegisteredRoute { callback: Box::from(callback) });
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

                    let response_obj = router.unwrap().get_mut(&meta.method);

                    if response_obj.is_none() {
                        self.send_status(&mut s, 405)?;
                        continue;
                    }

                    // For now, only 200 is allowed

                    send_response_object(&mut s, (response_obj.unwrap().callback)(0, Response { status: 200, body: String::new() }))?;

                    // s.write(b"HTTP/1.1 200 OK\r\nConnection: keep-alive\r\nContent-Type: application/json; charset=utf-8\r\nKeep-Alive: timeout=5\r\n\r\n{\"success\":\"")?;
    
                    // Manually route as GET /
                    // s.write(msg.unwrap().text.as_str().as_bytes())?;
                    // s.write(b"\"}")?;

                    s.flush()?;
                }
            }
        }

        Ok(())
    }
}
