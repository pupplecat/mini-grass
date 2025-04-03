use actix_web::{
    get,
    web::{self},
};
use serde::{Deserialize, Serialize};

use crate::transports::actix::error::ApiError;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[get("/health")]
pub async fn health() -> Result<web::Json<HealthResponse>, ApiError> {
    Ok(web::Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    }))
}
