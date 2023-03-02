use std::collections::HashMap;

use crate::{RequestPath, HTTP, HTTPVersion, Request};

pub fn parse_request(message: &str) -> Option<Request> {
    let mut lines = message.lines();

    let mut req_line = lines.next()?;

    if req_line.trim().is_empty() {
        // Skip an empty first line
        // https://datatracker.ietf.org/doc/html/rfc9112#section-2.2-6
        req_line = lines.next()?;
    }

    // Parse request line
    // method target version

    let mut req_fields = req_line.split(" ");

    let method = match req_fields.next()? {
        "GET" => HTTP::GET,
        "POST" => HTTP::POST,
        "PUT" => HTTP::PUT,
        "PATCH" => HTTP::PATCH,
        "DELETE" => HTTP::DELETE,
        "HEAD" => HTTP::HEAD,
        "OPTIONS" => HTTP::OPTIONS,
        _ => HTTP::UNRECOGNIZED
    };

    // TODO: Deal with dynamic routes?
    let target = req_fields.next()?;

    let version = match req_fields.next()? {
        "HTTP/1" => HTTPVersion::One,
        "HTTP/1.1" => HTTPVersion::OnePointOne,
        "HTTP/2" => HTTPVersion::Two,
        _ => HTTPVersion::Unknown
    };

    // Parse headers

    // Parse body (ignored for now)
    
    Some(
        Request {
            path: RequestPath { raw: target.to_string() },
            method,
            version,
            headers: HashMap::<String, String>::new()
        }
    )
}


