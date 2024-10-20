use std::collections::HashMap;

use rabbitmq_messages_management::{
    constants::RABBITMQ_MANAGEMENT_ROOT, exceptions::ServerError, prepare_authorization_headers,
    prepare_url, send_get,
};
use serde::{Deserialize, Serialize};

/// Represents a RabbitMQ exchange
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Exchange {
    /// Arguments for Binding
    #[serde(skip_deserializing)]
    arguments: Option<HashMap<String, String>>,
    /// Auto delete settings for the exchange
    auto_delete: bool,
    /// Durability of the exchange
    durable: bool,
    /// Is the exchange internal?
    internal: bool,
    /// Name of the exchange
    name: String,
    /// Type of the exchange
    r#type: String,
    user_who_performed_action: String,
    /// Vhost the exchange belongs to
    vhost: String,
}

/// Asynchronously retrieves a list of exchanges for a specified virtual host in RabbitMQ.
///
/// # Parameters
/// - `vhost`: A string slice representing the name of the virtual host for which exchanges are to be fetched.
///
/// # Returns
/// This function returns a `Result<Vec<Exchange>, ServerError>`. On success, it provides a vector of `Exchange` objects
/// associated with the specified virtual host. On failure, it returns a `ServerError` indicating what went wrong.
///
/// # Errors
/// The function may fail if:
/// - The environment variable `RABBITMQ_MANAGEMENT_ROOT` is not set, which results in a panic.
/// - The HTTP GET request to retrieve the exchanges fails for any reason (e.g., network issues, server errors).
///
/// # Example
/// ```rust
/// # #[cfg(doctest)] {
/// let vhost = "my_vhost";
/// match get_exchanges_for_vhost(vhost).await {
///     Ok(exchanges) => {
///         for exchange in exchanges {
///             println!("{:?}", exchange);
///         }
///     },
///     Err(error) => {
///         eprintln!("Error retrieving exchanges: {:?}", error);
///     },
/// }
/// # }
/// ```
///
/// # Notes
/// This function relies on the `dotenv` crate to access the environment variable and on an external function,
/// `send_get`, to perform the actual HTTP request. Make sure these dependencies are properly set up in your project.
pub(crate) async fn get_exchanges_for_vhost(vhost: &str) -> Result<Vec<String>, ServerError> {
    let root = &dotenv::var(RABBITMQ_MANAGEMENT_ROOT).expect("RABBITMQ_MANAGEMENT_ROOT not set.");
    let url = prepare_url(&root, &format!("api/exchanges/{}", vhost)).unwrap();
    let exchanges_response: Result<Vec<Exchange>, ()> =
        send_get(&url, Some(&prepare_authorization_headers())).await;

    match exchanges_response {
        Ok(exchanges) => Ok(exchanges.iter().map(|e| e.name.clone()).collect()),
        Err(err) => Err(ServerError::new(format!("{:?}", err))),
    }
}
