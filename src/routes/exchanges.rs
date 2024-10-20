use rocket::serde::json::Json;

use crate::rabbitmq::exchanges::get_exchanges_for_vhost;

#[get("/<vhost>")]
pub async fn exchanges(vhost: &str) -> Json<Vec<String>> {
    let exchanges = get_exchanges_for_vhost(vhost).await;
    match exchanges {
        Ok(response) => Json(response),
        Err(e) => {
            log::error!("{:?}", e.message);
            Json(vec![])
        }
    }
}
