#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Test"
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}