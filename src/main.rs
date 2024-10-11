use routes::queues::queues;

#[macro_use]
extern crate rocket;

mod constants;
mod rabbitmq;
mod routes;
mod transport;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/queues", routes![queues])
}
