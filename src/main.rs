use std::collections::HashMap;

use rocket::{fs::FileServer, serde::json::Json, Request};
use routes::{
    queues::{messages, queues},
    vhosts::vhosts,
};

#[macro_use]
extern crate rocket;

mod constants;
mod rabbitmq;
mod routes;

#[catch(500)]
fn internal_error() -> Json<HashMap<String, String>> {
    Json(HashMap::from([(
        "reason".to_string(),
        "Server error. Please try again later".to_string(),
    )]))
}

#[catch(400)]
fn bad_request(req: &Request) -> Json<HashMap<String, String>> {
    Json(HashMap::from([(
        "reason".to_string(),
        "Bad request".to_string(),
    )]))
}

#[catch(404)]
fn not_found(req: &Request) -> Json<HashMap<String, String>> {
    Json(HashMap::from([(
        "reason".to_string(),
        format!("{:?} not found.", req.uri().path()),
    )]))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/queues", routes![queues, messages])
        .mount("/vhosts", routes![vhosts])
        .register("/queues", catchers![internal_error, not_found, bad_request])
        .register("/vhosts", catchers![internal_error, not_found, bad_request])
}
