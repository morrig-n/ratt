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
    let mut headers = HashMap::<String, String>::new(); 

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }  

        // Parse header
        // TODO: Maybe get a list of allowed headers?
        
        let mut header = line.trim().split(":");

        let field_name = header.next();
        if field_name.is_none() || field_name.unwrap().is_empty() { continue; }

        let field_value = header.next();
        if field_value.is_none() || field_value.unwrap().is_empty() { continue; }

        headers.insert(field_name.unwrap().trim().to_string(), field_value.unwrap().trim().to_string());
    }

    // Parse body (ignored for now)
    
    Some(
        Request {
            path: RequestPath { raw: target.to_string() },
            method,
            version,
            headers
        }
    )
}


