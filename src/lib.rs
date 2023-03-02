pub mod parser;

use std::collections::HashMap;
use std::net::{TcpListener, Shutdown, TcpStream};
use std::io::{Read, Write};

pub struct App {
    registered_routes: HashMap<String, HashMap<HTTP, RegisteredRoute>>
}

pub struct RegisteredRoute {
    callback: Box<dyn FnMut(Request, Response) -> Response>
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum HTTP {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    UNRECOGNIZED
}

#[derive(Debug)]
pub enum HTTPVersion {
    One,
    OnePointOne,
    Two,
    Unknown,
}

#[derive(Debug)]
pub struct RequestPath {
    pub raw: String,
    pub absolute: String,
    pub query: HashMap<String, String>
}

#[derive(Debug)]
pub struct Request {
    pub path: RequestPath,
    pub method: HTTP,
    pub version: HTTPVersion,
    pub headers: HashMap<String, String>,
}

pub struct Response {
    // TODO: Maybe move this to an Enum?
    status: usize,
    body: String,
    headers: HashMap<String, String>
}

impl Response {
    pub fn set_status(mut self, status: usize) -> Self {
        self.status = status;
        self
    }

    pub fn send(mut self, content: String) -> Self {
        self.body = content;
        self
    }

    pub fn set_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key.to_lowercase(), value);
        self
    }
}

fn send_response_object(stream: &mut TcpStream, response: Response) -> std::io::Result<()> {
    stream.write(b"HTTP/1.1 ")?; 
    stream.write(match response.status {
        200 => b"200 OK",
        201 => b"201 Created",
        400 => b"400 Bad Request",
        404 => b"404 Not Found",
        418 => b"418 I'm a teapot",
        500 => b"500 Internal Server Error",
        _ => b"200 OK"
    })?;

    stream.write(b"\r\n")?;
    response.headers.iter().for_each(|(key, val)| {
        stream.write(format!("{}: {}\r\n", key, val).as_bytes()).unwrap();
    });
    stream.write(b"\r\n")?;
    stream.write(response.body.as_bytes())?;

    Ok(())
}

impl App {
    pub fn new() -> Self {
        App {
            registered_routes: HashMap::<String, HashMap<HTTP, RegisteredRoute>>::new()
        }
    }

    pub fn register(&mut self, path: &str, method: HTTP, callback: impl FnMut(Request, Response) -> Response + 'static) { 
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
            400 => "400 Bad Request",
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

    // TODO: Listen shouldn't really return Err at any point, since that
    // has crashed before on BrokenPipe
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

                    let maybe_request = parser::parse_request(&message);

                    if maybe_request.is_none() {
                        self.send_status(&mut s, 400)?;
                        continue;
                    }

                    let request = maybe_request.unwrap();

                    let router = self.registered_routes.get_mut(&request.path.absolute);

                    if router.is_none() {
                        self.send_status(&mut s, 404)?;
                        continue;
                    }

                    let response_obj = router.unwrap().get_mut(&request.method);

                    if response_obj.is_none() {
                        self.send_status(&mut s, 405)?;
                        continue;
                    }

                    let mut headers = HashMap::<String, String>::new();
                    headers.insert("content-type".to_string(), "text/plain; charset=utf-8".to_string());
                    headers.insert("connection".to_string(), "keep-alive".to_string());
                    headers.insert("keep-alive".to_string(), "timeout=5".to_string());

                    let res = Response { status: 200, body: String::new(), headers };
                    send_response_object(&mut s, (response_obj.unwrap().callback)(request, res))?;

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
