use rocket::serde::json::Json;

use crate::rabbitmq::vhosts::{get_vhosts, ResponseForQueryingVhosts};

#[get("/")]
pub async fn vhosts() -> Json<Vec<ResponseForQueryingVhosts>> {
    match get_vhosts().await {
        Ok(response) => Json(response),
        Err(e) => {
            log::error!("{:?}", e.message);
            Json(vec![])
        }
    }
}
