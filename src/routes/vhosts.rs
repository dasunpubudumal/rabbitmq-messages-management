use rocket::serde::json::Json;

use crate::rabbitmq::vhosts::{get_vhosts, ResponseForQueryingVhosts};

#[get("/vhosts")]
pub async fn vhosts() -> Json<Vec<ResponseForQueryingVhosts>> {
    Json(get_vhosts().await.unwrap())
}
