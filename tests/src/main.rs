use ratt;

fn main() {
    let mut app = ratt::App::new();

    app.register("/", ratt::HTTP::GET, |_req, _res| {
        "This message is brought to you by the register callback!".to_string()
    });

    app.register("/alternate-route", ratt::HTTP::GET, |_req, _res| {
        "Here's an alternative route!".to_string()
    });

    app.listen(":8000").unwrap();
}
