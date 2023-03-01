use ratt;

fn main() {
    let mut app = ratt::App::new();

    app.register("/", ratt::HTTP::GET, |req, res| {
        "This message is brought to you by the register callback!".to_string()
    });

    app.register("/alternate-route", ratt::HTTP::GET, |req, res| {
        "Here's an alternative route!".to_string()
    });

    app.listen(":8080").unwrap();
}
