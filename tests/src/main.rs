use ratt;

fn main() {
    let mut app = ratt::App::new();

    app.register("/", ratt::HTTP::GET, |_req, res| {
        res.set_status(200).send("This message is brought to you by the register callback!".to_string())
    });

    app.register("/alternate-route", ratt::HTTP::GET, |_req, res| {
        res.set_status(200).send("This is an alternative route!".to_string())
    });

    app.register("/create", ratt::HTTP::POST, |_req, res| {
        res.set_status(201).send("Created!".to_string())
    });

    app.listen(":8000").unwrap();
}
