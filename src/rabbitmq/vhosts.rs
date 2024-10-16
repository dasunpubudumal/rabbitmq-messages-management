use std::collections::HashMap;

use rabbitmq_messages_management::exceptions::ServerError;
use rabbitmq_messages_management::{
    constants::RABBITMQ_MANAGEMENT_ROOT, prepare_authorization_headers, prepare_url, send_get,
};
use serde::{Deserialize, Serialize};

/// Represents metadata for a RabbitMQ virtual host.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Metadata {
    /// Description of the metadata.
    description: String,
    /// Tags associated with the metadata.
    tags: Vec<String>,
}

/// Represents a RabbitMQ virtual host.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct RabbitMQVhost {
    /// Cluster state of the virtual host.
    cluster_state: HashMap<String, String>,
    /// Description of the virtual host.
    description: String,
    /// Metadata associated with the virtual host.
    metadata: Metadata,
    /// Name of the virtual host.
    name: String,
    /// Tags associated with the virtual host.
    tags: Vec<String>,
    /// Indicates if tracing is enabled for the virtual host.
    tracing: bool,
}

/// Represents vhost names internally
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct ResponseForQueryingVhosts {
    /// Name of vhost
    name: String,
}

/// Retrieves the list of virtual hosts from the RabbitMQ management API.
///
/// This function sends an HTTP GET request to the RabbitMQ management API to retrieve the list of virtual hosts.
/// The response is deserialized into a vector of `RabbitMQVhost` structs, and the names of these virtual hosts
/// are returned as a vector of `ResponseForQueryingVhosts` structs.
///
/// # Returns
///
/// * `Result<Vec<ResponseForQueryingVhosts>, ()>` - On success, returns a vector of `ResponseForQueryingVhosts` structs. On failure, returns an empty tuple `()`.
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     match get_vhosts().await {
///         Ok(vhosts) => {
///             for vhost in vhosts {
///                 println!("Virtual Host Name: {}", vhost.name);
///             }
///         }
///         Err(_) => println!("Failed to retrieve virtual hosts"),
///     }
/// }
/// ```
pub async fn get_vhosts() -> Result<Vec<ResponseForQueryingVhosts>, ServerError> {
    let root = &dotenv::var(RABBITMQ_MANAGEMENT_ROOT).expect("RABBITMQ_MANAGEMENT_ROOT not set");
    let url = prepare_url(&root, "api/vhosts").unwrap();
    let vhost_responses: Result<Vec<RabbitMQVhost>, ()> =
        send_get(&url, Some(&prepare_authorization_headers())).await;

    match vhost_responses {
        Ok(vhosts) => Ok(vhosts
            .iter()
            .map(|vhost| ResponseForQueryingVhosts {
                name: vhost.name.clone(),
            })
            .collect()),
        Err(e) => Err(ServerError::new(format!("{:?}", e))),
    }
}
