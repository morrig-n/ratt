use std::collections::HashMap;

use crate::{RequestPath, HTTP, HTTPVersion, Request};

fn parse_path(path: &str) -> Option<RequestPath> {
    // We only support relative paths currently
    // e.g. /, /abc, /abc?d=4&e=3
    let mut query = HashMap::<String, String>::new();

    match path.split_once("?") {
        Some((absolute, search)) => {
            search.split("&").for_each(|param| {
                if let Some((key, value)) = param.split_once("=") {
                    query.insert(key.to_string(), value.to_string());
                }
            });

            Some(RequestPath { raw: path.to_string(), absolute: absolute.to_string(), query })
        },
        None => {
            Some(RequestPath { raw: path.to_string(), absolute: path.to_string(), query })
        }
    }
}

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
    let path = parse_path(req_fields.next()?)?;

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

    Some(
        Request {
            path,
            method,
            version,
            headers
        }
    )
}


