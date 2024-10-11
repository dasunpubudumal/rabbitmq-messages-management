#[macro_use]
extern crate rocket;

mod constants;
mod routes;

mod rabbitmq;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world])
}
