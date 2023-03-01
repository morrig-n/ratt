pub mod parser;

use std::collections::HashMap;
use std::net::{TcpListener, Shutdown};
use std::io::{Read, Write};

pub struct App {
    registered_routes: HashMap<RouteKey, RegisteredRoute>
}

#[derive(PartialEq, Eq, Hash)]
pub struct RouteKey {
    method: HTTP,
    path: String
}

pub struct RegisteredRoute {
    text: String    
}

#[derive(PartialEq, Eq, Hash)]
pub enum HTTP {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS
}

impl App {
    pub fn new() -> Self {
        App {
            registered_routes: HashMap::<RouteKey, RegisteredRoute>::new()
        }
    }

    pub fn register<T>(&mut self, path: &str, method: HTTP, mut callback: T) where T: FnMut(i8, i8) -> String { 
        self.registered_routes.insert(RouteKey {
            method,
            path: path.to_string()
        }, RegisteredRoute { text: callback(1, 2) }); 
    }

    pub fn listen(&mut self, _port: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind("localhost:8080")?;

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

                    let method: HTTP = parser::get_method(&message).unwrap(); 

                    let msg = self.registered_routes.get(&RouteKey {
                        method,
                        path: "/".to_string()
                    });

                    if msg.is_none() {
                        s.write(b"HTTP/1.1 405 Method Not Allowed\r\n\r\n")?;
                        s.flush()?;
                        s.shutdown(Shutdown::Both)?;
                        
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
