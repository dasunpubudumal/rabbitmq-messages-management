use crate::constants::RABBITMQ_MANAGEMENT_ROOT;
use rabbitmq_messages_management::{prepare_authorization_headers, prepare_url, send_get};
use serde::{Deserialize, Serialize};

// Represents a RabbitMQ queue.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Queue {
    /// Arguments for the queue.
    arguments: Arguments,
    /// Indicates if the queue is auto-deleted when no longer used.
    auto_delete: bool,
    /// Status of the backing queue.
    backing_queue_status: BackingQueueStatus,
    /// Capacity of the consumer.
    consumer_capacity: f64,
    /// Utilization of the consumer.
    consumer_utilisation: f64,
    /// Number of consumers.
    consumers: u64,
    /// Indicates if the queue is durable.
    durable: bool,
    /// Effective policy definition for the queue.
    effective_policy_definition: EffectivePolicyDefinition,
    /// Indicates if the queue is exclusive.
    exclusive: bool,
    /// Tag of the exclusive consumer, if any.
    exclusive_consumer_tag: Option<String>,
    /// Garbage collection settings for the queue.
    garbage_collection: GarbageCollection,
    /// Timestamp of the head message, if any.
    head_message_timestamp: Option<String>,
    /// Time since the queue has been idle.
    idle_since: String,
    /// Memory used by the queue.
    memory: u64,
    /// Total bytes of messages in the queue.
    message_bytes: u64,
    /// Bytes of messages paged out.
    message_bytes_paged_out: u64,
    /// Bytes of persistent messages.
    message_bytes_persistent: u64,
    /// Bytes of messages in RAM.
    message_bytes_ram: u64,
    /// Bytes of ready messages.
    message_bytes_ready: u64,
    /// Bytes of unacknowledged messages.
    message_bytes_unacknowledged: u64,
    /// Total number of messages in the queue.
    messages: u64,
    /// Details of the messages.
    messages_details: MessageDetails,
    /// Number of messages paged out.
    messages_paged_out: u64,
    /// Number of persistent messages.
    messages_persistent: u64,
    /// Number of messages in RAM.
    messages_ram: u64,
    /// Number of ready messages.
    messages_ready: u64,
    /// Details of the ready messages.
    messages_ready_details: MessageDetails,
    /// Number of ready messages in RAM.
    messages_ready_ram: u64,
    /// Number of unacknowledged messages.
    messages_unacknowledged: u64,
    /// Details of the unacknowledged messages.
    messages_unacknowledged_details: MessageDetails,
    /// Number of unacknowledged messages in RAM.
    messages_unacknowledged_ram: u64,
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
    reductions: u64,
    /// Details of the reductions.
    reductions_details: ReductionsDetails,
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

/// Fetches the details of a specific queue for a given virtual host.
///
/// This function sends an HTTP GET request to the RabbitMQ management API to retrieve the details
/// of queues in specified virtual host. The function uses the `send_get` function
/// to perform the HTTP request and deserializes the response into a vector of `Queue` structs.
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
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     use rabbitmq_messages_management::rabbitmq::queues::get_queue_for_vhost;
///     let vhost = "my_vhost";
///     let queues = get_queue_for_vhost(vhost).await.unwrap();
///     println!("{:?}", queues);
///     Ok(())
/// }
/// ```
pub async fn get_queue_for_vhost(vhost: &str) -> Result<Vec<Queue>, ()> {
    // TODO: Error handling
    let root = &dotenv::var(RABBITMQ_MANAGEMENT_ROOT).expect("RABBITMQ_MANAGEMENT_ROOT not set");
    let url = prepare_url(&root, &format!("api/queues/{}", vhost)).unwrap();
    let queues: Vec<Queue> = send_get(&url, Some(&prepare_authorization_headers()))
        .await
        .unwrap();
    Ok(queues)
}
