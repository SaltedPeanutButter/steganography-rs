#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello there! It looks like you found my API entry point. Do note that the API is private, please do not attempt to use :)"
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}