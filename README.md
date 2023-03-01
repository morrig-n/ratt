# Ratt

__Ratt__ is a dependency-less HTTP server library written in Rust.

This library is **work-in-progress**. Use at your own risk.

## Current State

This library is very unfinished at present.

Currently it supports returning a simple message on a GET request.

It is very speciailised without much error handling currently, as this has only just begun development.

## Usage

```rust
fn main() {
    let mut app = ratt::App::new();

    app.register("/", ratt::HTTP::GET, | req, res | {
        "A message!".to_string()
    });

    app.listen(":8080");
}
```
