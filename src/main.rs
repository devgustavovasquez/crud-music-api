use rocket::http::Status;
use rocket::response::{content, status};

#[macro_use] extern crate rocket;

#[get("/hello-world")]
fn index() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(
        Status::Ok,
        content::RawJson("{\"message\": \"Hello, world!\"}"),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}