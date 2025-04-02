use std::sync::Arc;

use actix_web::{get, post, web, HttpResponse};
use display_json::DebugAsJson;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{container::ServiceContext, transports::actix::error::ApiError};

#[post("/bandwidth")]
pub async fn report_bandwidth(
    ctx: web::Data<ServiceContext>,
    request: web::Json<ReportBandwidthRequest>,
) -> Result<web::Json<ReportBandwidthResponse>, ApiError> {
    let response = do_report_bandwidth(ctx.into_inner(), request.into_inner()).await?;
    Ok(web::Json(response))
}

#[instrument(skip(ctx), ret, err)]
async fn do_report_bandwidth(
    ctx: Arc<ServiceContext>,
    request: ReportBandwidthRequest,
) -> Result<ReportBandwidthResponse, ApiError> {
    let pending_sync = ctx
        .report_bandwidth_usecase()
        .report(request.node_id, request.bandwidth)
        .await?;

    Ok(ReportBandwidthResponse {
        node_id: request.node_id,
        status: "recorded".to_string(),
        pending_sync,
    })
}

#[derive(DebugAsJson, Serialize, Deserialize)]
pub struct ReportBandwidthRequest {
    node_id: u64,
    bandwidth: u64,
}

#[derive(DebugAsJson, Serialize, Deserialize)]
pub struct ReportBandwidthResponse {
    node_id: u64,
    status: String,
    pending_sync: u64,
}
