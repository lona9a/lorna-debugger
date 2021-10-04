#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello"
    /* Implement easy to use GUI and Tree like in GHIDRA or IDA */
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
