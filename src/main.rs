use routes::{
    queues::{messages, queues},
    vhosts::vhosts,
};

#[macro_use]
extern crate rocket;

mod constants;
mod rabbitmq;
mod routes;
mod transport;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/queues", routes![queues, messages])
        .mount("/vhosts", routes![vhosts])
}
