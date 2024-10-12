use crate::rabbitmq::queues::{
    get_messages_from_a_queue, get_queue_for_vhost, MessageRetrievalRequest, Queue, RabbitMQMessage,
};
use rocket::serde::json::Json;

#[get("/<vhost>")]
pub async fn queues(vhost: &str) -> Json<Vec<Queue>> {
    Json(get_queue_for_vhost(vhost).await.unwrap())
}

#[get("/<vhost>/<queue_name>?<count>")]
pub async fn messages(vhost: &str, queue_name: &str, count: &str) -> Json<Vec<RabbitMQMessage>> {
    let number: u64 = count
        .parse::<u64>()
        .expect("Failed to parse string to a valid count");
    Json(
        get_messages_from_a_queue(vhost.to_string(), queue_name.to_string(), number)
            .await
            .unwrap(),
    )
}
