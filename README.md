# Ratt

__Ratt__ is a dependency-less HTTP server library written in Rust.

This library is **work-in-progress**. Use at your own risk.

## Current State

This library is very unfinished at present.

Currently, it supports returning a simple message on requests to static routes (e.g. GET /, POST /more).

It is very speciailised without much error handling currently, as this has only just begun development.

## Usage

```rust
fn main() {
    let mut app = ratt::App::new();

    app.register("/", ratt::HTTP::GET, | _req, res | {
        res.send("Some text!".to_string())
    });

    app.register("/post-example", ratt::HTTP::POST, | _req, res | {
        res.set_status(201).send("Created successfully!".to_string())      
    });

    app.listen(":8080").unwrap();
}
```
