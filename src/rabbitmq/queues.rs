use crate::constants::RABBITMQ_MANAGEMENT_ROOT;
use rabbitmq_messages_management::{prepare_authorization_headers, send_get};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct Queue {
    arguments: Arguments,
    auto_delete: bool,
    backing_queue_status: BackingQueueStatus,
    consumer_capacity: u64,
    consumer_utilisation: f64,
    consumers: u64,
    durable: bool,
    effective_policy_definition: EffectivePolicyDefinition,
    exclusive: bool,
    exclusive_consumer_tag: Option<String>,
    garbage_collection: GarbageCollection,
    head_message_timestamp: Option<String>,
    idle_since: String,
    memory: u64,
    message_bytes: u64,
    message_bytes_paged_out: u64,
    message_bytes_persistent: u64,
    message_bytes_ram: u64,
    message_bytes_ready: u64,
    message_bytes_unacknowledged: u64,
    messages: u64,
    messages_details: MessageDetails,
    messages_paged_out: u64,
    messages_persistent: u64,
    messages_ram: u64,
    messages_ready: u64,
    messages_ready_details: MessageDetails,
    messages_ready_ram: u64,
    messages_unacknowledged: u64,
    messages_unacknowledged_details: MessageDetails,
    messages_unacknowledged_ram: u64,
    name: String,
    node: String,
    operator_policy: Option<String>,
    policy: Option<String>,
    recoverable_slaves: Option<String>,
    reductions: u64,
    reductions_details: ReductionsDetails,
    single_active_consumer_tag: Option<String>,
    state: String,
    #[serde(rename = "type")]
    queue_type: String,
    vhost: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Arguments {}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BackingQueueStatus {
    avg_ack_egress_rate: f64,
    avg_ack_ingress_rate: f64,
    avg_egress_rate: f64,
    avg_ingress_rate: f64,
    delta: (String, u64, u64, u64, u64),
    len: u64,
    mode: String,
    next_seq_id: u64,
    q1: u64,
    q2: u64,
    q3: u64,
    q4: u64,
    target_ram_count: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct EffectivePolicyDefinition {}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GarbageCollection {
    fullsweep_after: u64,
    max_heap_size: u64,
    min_bin_vheap_size: u64,
    min_heap_size: u64,
    minor_gcs: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MessageDetails {
    rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReductionsDetails {
    rate: f64,
}

/// Fetches the details of a specific queue for a given virtual host.
///
/// This function sends an HTTP GET request to the RabbitMQ management API to retrieve the details
/// of a specific queue for the specified virtual host. The function uses the `send_get` function
/// to perform the HTTP request and deserializes the response into a vector of `Queue` structs.
///
/// # Arguments
///
/// - `vhost`: A string slice that holds the name of the virtual host.
/// - `queue_name`: A string slice that holds the name of the queue.
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
///     use rabbitmq_messages_management::rabbitmq::queues::get_queue_for_vhost;
///     let vhost = "my_vhost";
///     let queue_name = "my_queue";
///     let queues = get_queue_for_vhost(vhost, queue_name).await.unwrap();
///     println!("{:?}", queues);
///     Ok(())
/// }
/// ```
pub async fn get_queue_for_vhost(vhost: &str, queue_name: &str) -> Result<Vec<Queue>, ()> {
    let queues: Vec<Queue> = send_get(
        &dotenv::var(RABBITMQ_MANAGEMENT_ROOT).expect("RABBITMQ_MANAGEMENT_ROOT not set"),
        Some(&prepare_authorization_headers()),
    )
    .await
    .unwrap();
    Ok(queues)
}
