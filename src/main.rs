#[macro_use]
extern crate dotenv_codegen;

use rabbitmq_messages_management::{prepare_url, send_get};
use serde::{Deserialize, Serialize};

mod rabbitmq;

#[derive(Serialize, Deserialize, Debug)]
struct Ip {
    origin: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let result: Ip = send_get(
        &prepare_url("http://httpbin.org", "ip").unwrap(),
        None
    ).await.unwrap();

    println!("{:?}", result);

    Ok(())
}
