use crate::rabbitmq::queues::{
    get_messages_from_a_queue, get_queue_for_vhost, Queue, ResponseForQueryingMessages,
};
use rocket::serde::json::Json;

#[get("/<vhost>")]
pub async fn queues(vhost: &str) -> Json<Vec<Queue>> {
    let get_queue_for_vhost = get_queue_for_vhost(vhost).await;
    match get_queue_for_vhost {
        Ok(response) => Json(response),
        Err(e) => {
            log::error!("{:?}", e.message);
            Json(vec![])
        }
    }
}

#[get("/<vhost>/<queue_name>?<count>")]
pub async fn messages(
    vhost: &str,
    queue_name: &str,
    count: &str,
) -> Json<Vec<ResponseForQueryingMessages>> {
    let number: u64 = count
        .parse::<u64>()
        .expect("Failed to parse string to a valid count");
    let messages =
        get_messages_from_a_queue(vhost.to_string(), queue_name.to_string(), number).await;
    match messages {
        Ok(response) => Json(response),
        Err(e) => {
            log::error!("{:?}", e.message);
            Json(vec![])
        }
    }
}
