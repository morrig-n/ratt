use crate::{ HTTP, HTTPVersion };

// Utilities for parsing HTTP requests

pub struct Meta {
    pub method: HTTP,
    pub path: String,
    pub version: HTTPVersion
}

pub fn parse_http_meta(msg: &str) -> Option<Meta> {
    let meta_line = msg.split("\r\n").next();

    match meta_line {
        None => None,
        Some(line) => {
            let parts: Vec<&str> = line.split(" ").map(|x| x.trim()).collect();

            if parts.len() < 3 {
                // Must have all 3
                return None;
            }

            let method: Option<HTTP> = match parts[0].to_uppercase().as_str() {
                "GET" => Some(HTTP::GET),
                "POST" => Some(HTTP::POST),
                "PUT" => Some(HTTP::PUT),
                "PATCH" => Some(HTTP::PATCH),
                "DELETE" => Some(HTTP::DELETE),
                "OPTIONS" => Some(HTTP::OPTIONS),
                "HEAD" => Some(HTTP::HEAD),
                _ => None
            };

            if method.is_none() {
                // Unknown method!
                return None;
            }

            let path = parts[1].to_string();

            let version: Option<HTTPVersion> = match parts[2].to_uppercase().as_str() {
                "HTTP/1" => Some(HTTPVersion::One),
                "HTTP/1.1" => Some(HTTPVersion::OnePointOne),
                "HTTP/2" => Some(HTTPVersion::Two),
                _ => None
            };

            if version.is_none() {
                // Unknown HTTP version (we don't support v3 yet)
                return None;
            }

            return Some(
                Meta { method: method.unwrap(), path, version: version.unwrap() }
            );
        }
    }
}
