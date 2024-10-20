use std::collections::HashMap;

use crate::constants::RABBITMQ_MANAGEMENT_ROOT;
use rabbitmq_messages_management::{
    exceptions::ServerError, prepare_authorization_headers, prepare_url, send_get, send_post,
};
use serde::{Deserialize, Serialize};

// Represents a RabbitMQ queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Queue {
    /// Arguments for the queue.
    arguments: Option<Arguments>,
    /// Indicates if the queue is auto-deleted when no longer used.
    auto_delete: Option<bool>,
    /// Status of the backing queue.
    backing_queue_status: Option<BackingQueueStatus>,
    /// Capacity of the consumer.
    consumer_capacity: Option<f64>,
    /// Utilization of the consumer.
    consumer_utilisation: Option<f64>,
    /// Number of consumers.
    consumers: Option<u64>,
    /// Indicates if the queue is durable.
    durable: Option<bool>,
    /// Effective policy definition for the queue.
    effective_policy_definition: Option<EffectivePolicyDefinition>,
    /// Indicates if the queue is exclusive.
    exclusive: Option<bool>,
    /// Tag of the exclusive consumer, if any.
    exclusive_consumer_tag: Option<String>,
    /// Garbage collection settings for the queue.
    garbage_collection: Option<GarbageCollection>,
    /// Timestamp of the head message, if any.
    head_message_timestamp: Option<String>,
    /// Time since the queue has been idle.
    idle_since: Option<String>,
    /// Memory used by the queue.
    memory: Option<u64>,
    /// Total bytes of messages in the queue.
    message_bytes: Option<u64>,
    /// Bytes of messages paged out.
    message_bytes_paged_out: Option<u64>,
    /// Bytes of persistent messages.
    message_bytes_persistent: Option<u64>,
    /// Bytes of messages in RAM.
    message_bytes_ram: Option<u64>,
    /// Bytes of ready messages.
    message_bytes_ready: Option<u64>,
    /// Bytes of unacknowledged messages.
    message_bytes_unacknowledged: Option<u64>,
    /// Total number of messages in the queue.
    messages: Option<u64>,
    /// Details of the messages.
    messages_details: Option<MessageDetails>,
    /// Number of messages paged out.
    messages_paged_out: Option<u64>,
    /// Number of persistent messages.
    messages_persistent: Option<u64>,
    /// Number of messages in RAM.
    messages_ram: Option<u64>,
    /// Number of ready messages.
    messages_ready: Option<u64>,
    /// Details of the ready messages.
    messages_ready_details: Option<MessageDetails>,
    /// Number of ready messages in RAM.
    messages_ready_ram: Option<u64>,
    /// Number of unacknowledged messages.
    messages_unacknowledged: Option<u64>,
    /// Details of the unacknowledged messages.
    messages_unacknowledged_details: Option<MessageDetails>,
    /// Number of unacknowledged messages in RAM.
    messages_unacknowledged_ram: Option<u64>,
    /// Name of the queue.
    name: String,
    /// Node where the queue is located.
    node: String,
    /// Operator policy for the queue, if any.
    operator_policy: Option<String>,
    /// Policy for the queue, if any.
    policy: Option<String>,
    /// Recoverable slaves for the queue, if any.
    recoverable_slaves: Option<String>,
    /// Number of reductions.
    reductions: Option<u64>,
    /// Details of the reductions.
    reductions_details: Option<ReductionsDetails>,
    /// Tag of the single active consumer, if any.
    single_active_consumer_tag: Option<String>,
    /// State of the queue.
    state: String,
    /// Type of the queue.
    #[serde(rename = "type")]
    queue_type: String,
    /// Virtual host of the queue.
    vhost: String,
}

/// Represents the arguments for a RabbitMQ queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Arguments {}

/// Represents the status of the backing queue.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BackingQueueStatus {
    /// Average egress rate for acknowledgments.
    avg_ack_egress_rate: f64,
    /// Average ingress rate for acknowledgments.
    avg_ack_ingress_rate: f64,
    /// Average egress rate.
    avg_egress_rate: f64,
    /// Average ingress rate.
    avg_ingress_rate: f64,
    /// Delta values.
    #[serde(skip_deserializing)]
    delta: (String, u64, u64, u64, u64),
    /// Length of the queue.
    len: u64,
    /// Mode of the queue.
    mode: String,
    /// Next sequence ID.
    next_seq_id: u64,
    /// Q1 value.
    q1: u64,
    /// Q2 value.
    q2: u64,
    /// Q3 value.
    q3: u64,
    /// Q4 value.
    q4: u64,
    /// Target RAM count.
    target_ram_count: String,
}

/// Represents the effective policy definition for a RabbitMQ queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct EffectivePolicyDefinition {}

/// Represents the garbage collection settings for a RabbitMQ queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct GarbageCollection {
    /// Full sweep after this many collections.
    fullsweep_after: u64,
    /// Maximum heap size.
    max_heap_size: u64,
    /// Minimum binary virtual heap size.
    min_bin_vheap_size: u64,
    /// Minimum heap size.
    min_heap_size: u64,
    /// Number of minor garbage collections.
    minor_gcs: u64,
}

/// Represents the details of messages in a RabbitMQ queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct MessageDetails {
    /// Rate of messages.
    rate: f64,
}

/// Represents the details of reductions in a RabbitMQ queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct ReductionsDetails {
    /// Rate of reductions.
    rate: f64,
}

/// Represents the request for retrieving messages from a queue
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct MessageRetrievalRequest {
    /// Vhost of the queue
    vhost: String,
    /// Name of the queue
    name: String,
    /// For requeuing, do `ack_requeue_true`
    ackmode: String,
    /// Must be either "auto" (in which case the payload will be returned as a string if it is valid UTF-8,
    ///  and base64 encoded otherwise), or "base64" (in which case the payload will always be base64 encoded).
    encoding: String,
    /// Controls the maximum number of messages to get.
    ///  You may get fewer messages than this if the queue cannot immediately provide them.
    count: u64,
}

/// Represents the properties of a RabbitMQ message.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Properties {
    /// Priority of the message.
    priority: Option<u8>,
    /// Delivery mode of the message.
    delivery_mode: u8,
    /// Headers associated with the message.
    headers: HashMap<String, serde_json::Value>,
    /// Content type of the message.
    content_type: Option<String>,
}

/// Represents a RabbitMQ message.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RabbitMQMessage {
    /// Number of bytes in the payload.
    payload_bytes: u64,
    /// Indicates if the message was redelivered.
    redelivered: bool,
    /// Name of the exchange.
    exchange: String,
    /// Routing key used for the message.
    routing_key: String,
    /// Number of messages in the queue.
    message_count: u64,
    /// Properties of the message.
    properties: Properties,
    /// Payload of the message.
    payload: String,
    /// Encoding of the payload.
    payload_encoding: String,
}

/// API response for querying messages from a queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResponseForQueryingMessages {
    /// Payload from a queue
    payload: String,
    /// Encoding of the payload
    payload_encoding: String,
}

/// Fetches the details of a specific queue for a given virtual host.
///
/// This function sends an HTTP GET request to the RabbitMQ management API to retrieve the details
/// of queues in specified virtual host. The function uses the `send_get` function
/// to perform the HTTP request and deserializes the response into a vector of `Queue` structs.
///
///
/// # Arguments
///
/// - `vhost`: A string slice that holds the name of the virtual host.
///
/// # Returns
///
/// - `Result<Vec<Queue>, ()>`: On success, returns a vector of `Queue` structs. On failure, returns an empty tuple `()`.
///
/// # Example
///
/// ```rust
/// # #[cfg(doctest)] {
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     use rabbitmq_messages_management::rabbitmq::queues::get_queue_for_vhost;
///     let vhost = "my_vhost";
///     let queues = get_queue_for_vhost(vhost).await.unwrap();
///     println!("{:?}", queues);
///     Ok(())~
/// }
/// # }
/// ```
pub async fn get_queue_for_vhost(vhost: &str) -> Result<Vec<Queue>, ServerError> {
    let root = &dotenv::var(RABBITMQ_MANAGEMENT_ROOT).expect("RABBITMQ_MANAGEMENT_ROOT not set");
    let url = prepare_url(root, &format!("api/queues/{}", vhost)).unwrap();
    let queues_response: Result<Vec<Queue>, ()> =
        send_get(&url, Some(&prepare_authorization_headers())).await;

    match queues_response {
        Ok(queues) => Ok(queues),
        Err(e) => Err(ServerError::new(format!("{:?}", e))),
    }
}

/// Retrieves messages from a specified queue in a given virtual host.
///
/// This function sends an HTTP POST request to the RabbitMQ management API to retrieve messages
/// from a specified queue in a given virtual host. The request includes the virtual host, queue name,
/// and the number of messages to retrieve. The response is deserialized into a vector of `RabbitMQMessage` structs,
/// and the payloads of these messages are returned as a vector of `ResponseForQueryingMessages` structs.
///
/// # Arguments
///
/// * `vhost` - A string representing the virtual host from which to retrieve messages.
/// * `queue_name` - A string representing the name of the queue from which to retrieve messages.
/// * `count` - A u64 representing the number of messages to retrieve.
///
/// # Returns
///
/// * `Result<Vec<ResponseForQueryingMessages>, ()>` - On success, returns a vector of `ResponseForQueryingMessages` structs. On failure, returns an empty tuple `()`.
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let vhost = "my_vhost".to_string();
///     let queue_name = "my_queue".to_string();
///     let count = 10;
///
///     match get_messages_from_a_queue(vhost, queue_name, count).await {
///         Ok(messages) => {
///             for message in messages {
///                 println!("Payload: {}", message.payload);
///                 println!("Payload Encoding: {}", message.payload_encoding);
///                 println!("Content Type: {}", message.content_type);
///             }
///         }
///         Err(_) => println!("Failed to retrieve messages"),
///     }
/// }
/// ```
pub async fn get_messages_from_a_queue(
    vhost: String,
    queue_name: String,
    count: u64,
) -> Result<Vec<ResponseForQueryingMessages>, ServerError> {
    let root = &dotenv::var(RABBITMQ_MANAGEMENT_ROOT).expect("RABBITMQ_MANAGEMENT_ROOT not set");
    let url = prepare_url(root, &format!("api/queues/{}/{}/get", vhost, queue_name)).unwrap();
    let request = MessageRetrievalRequest {
        vhost,
        name: queue_name,
        ackmode: "ack_requeue_true".to_string(),
        encoding: "auto".to_string(),
        count,
    };
    // The following other types implement trait `From<T>`:
    //  ```
    //  <isahc::body::AsyncBody as From<&[u8]>>
    //  <isahc::body::AsyncBody as From<&str>>
    //  <isahc::body::AsyncBody as From<()>>
    //  <isahc::body::AsyncBody as From<Vec<u8>>>
    //  <isahc::body::AsyncBody as From<std::option::Option<T>>>
    //  <isahc::body::AsyncBody as From<std::string::String>>
    // ```
    //
    // Therefore, if the request body is not of type `&[u8]`, `&str`, `()`, `Vec<u8>`, or `std::string::String`
    //  make sure you implement it for the request body you're setting.
    // I suggest use serde_json::to_string to convert the struct to a string and use it as a body.

    let messages_response: Result<Vec<RabbitMQMessage>, ()> = send_post(
        &url,
        Some(&prepare_authorization_headers()),
        serde_json::to_string(&request).unwrap(),
    )
    .await;

    match messages_response {
        Ok(messages) => Ok(messages
            .iter()
            .map(|message| ResponseForQueryingMessages {
                payload: message.payload.clone(),
                payload_encoding: message.payload_encoding.clone(),
            })
            .collect()),
        Err(e) => Err(ServerError::new(format!("{:?}", e))),
    }
}
