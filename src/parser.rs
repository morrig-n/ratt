use crate::HTTP;

// Utilities for parsing HTTP requests

pub fn get_method(msg: &str) -> Option<HTTP> {
    let req_line = msg.split("\r\n\r\n").next();

    match req_line {
        None => None,
        Some(line) => {
            if line.len() < 3 || !line.chars().nth(0).unwrap().is_alphabetic() {
                return None;
            }

            match line {
                l if l.to_uppercase().starts_with("GET") => Some(HTTP::GET),
                l if l.to_uppercase().starts_with("POST") => Some(HTTP::POST),
                l if l.to_uppercase().starts_with("OPTIONS") => Some(HTTP::OPTIONS),
                l if l.to_uppercase().starts_with("PUT") => Some(HTTP::PUT),
                l if l.to_uppercase().starts_with("PATCH") => Some(HTTP::PATCH),
                l if l.to_uppercase().starts_with("DELETE") => Some(HTTP::DELETE),
                l if l.to_uppercase().starts_with("HEAD") => Some(HTTP::HEAD),
                _ => None
            }
        }
    }
}
