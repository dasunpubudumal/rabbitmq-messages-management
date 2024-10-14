use rocket::fs::FileServer;
use routes::{
    queues::{messages, queues},
    vhosts::vhosts,
};

#[macro_use]
extern crate rocket;

mod constants;
mod rabbitmq;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/queues", routes![queues, messages])
        .mount("/vhosts", routes![vhosts])
}
