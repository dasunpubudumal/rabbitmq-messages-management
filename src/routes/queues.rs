use crate::rabbitmq::queues::{get_queue_for_vhost, Queue};
use rocket::serde::{json::Json, Serialize};

#[get("/<vhost>")]
pub async fn queues(vhost: &str) -> Json<Vec<Queue>> {
    Json(get_queue_for_vhost(vhost).await.unwrap())
}
