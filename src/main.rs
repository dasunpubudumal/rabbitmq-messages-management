#[macro_use]
extern crate dotenv_codegen;

use std::collections::HashMap;
use base64::prelude::*;

use rabbitmq_messages_management::{prepare_url, send_get};
use serde::{Deserialize, Serialize};

mod rabbitmq;

#[derive(Serialize, Deserialize, Debug)]
struct Ip {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result: Ip = send_get(
        &prepare_url("http://httpbin.org", "ip").unwrap(),
        None,
    )
    .await
    .unwrap();

    let management_root = dotenv!("RABBITMQ_MANAGEMENT_ROOT");
    let management_username = dotenv!("RABBITMQ_MANAGEMENT_USERNAME");
    let management_pw = dotenv!("RABBITMQ_MANAGEMENT_PASSWORD");

    let map = &HashMap::from([(
        "Authorization".to_string(),
        format!(
            "Basic {}",
            BASE64_STANDARD.encode(format!("{}:{}", management_username, management_pw))
        ),
    )]);

    let rabbitmq_result: String = send_get(management_root, Some(map)).await.unwrap();

    println!("{:?}", rabbitmq_result);

    Ok(())
}
