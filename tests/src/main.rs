use ratt;

fn main() {
    let mut app = ratt::App::new();

    app.register("/", ratt::HTTP::GET, |_req, res| {
        res.set_status(200).send("This message is brought to you by the register callback!".to_string())
    });

    app.register("/coffee-please", ratt::HTTP::GET, |_req, res| {
        res.set_status(418).send("I'm a teapot!".to_string())
    });

    app.register("/search", ratt::HTTP::GET, |req, res| {
        let search = req.path.query.get("s");
        if let Some(s) = search {
            res.send(format!("You searched for: {}", s.to_string()))
        } else {
            res.send("You didn't provide a search!".to_string())
        }
    });

    app.register("/create", ratt::HTTP::POST, |_req, res| {
        res.set_status(201).send("Created!".to_string())
    });

    app.register("/json" ,ratt::HTTP::GET, |_req, res| {
        res.set_header("Content-Type".to_string(), "application/json".to_string()).send("{\"example\": 2}".to_string())
    });

    app.listen(":8000").unwrap();
}
