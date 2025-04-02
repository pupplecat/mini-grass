use std::{
    collections::HashMap,
    io::Result,
    sync::{Arc, Mutex},
};

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::{container::ServiceContext, core::config::Config};

pub async fn serve(config: &Config) -> Result<()> {
    let service_context = ServiceContext::new(config);

    let server = actix_serve(service_context, &config.listening_address())?;
    server.await?;
    Ok(())
}

fn actix_serve<T>(service_context: T, address: &String) -> std::io::Result<Server>
where
    T: Sync + Send + Clone + 'static,
{
    let ctx = Arc::from(service_context.clone());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(ctx.clone())
            .route("/report", web::post().to(report_bw))
    })
    .bind(address)?;

    println!("service bound address: {}", address);

    Ok(server.run())
}

async fn report_bw(data: web::Json<NodeReport>, state: web::Data<AppState>) -> impl Responder {
    let mut contribs = state.contributions.lock().unwrap();
    let total = contribs.entry(data.node_id.clone()).or_insert(0);
    *total += data.bandwidth;
    HttpResponse::Ok().json(ReportResponse {
        node_id: data.node_id.clone(),
        status: "recorded".to_string(),
        total_bw: *total,
    })
}

#[derive(Deserialize)]
struct NodeReport {
    node_id: String,
    bandwidth: u64,
}

#[derive(Serialize)]
struct ReportResponse {
    node_id: String,
    status: String,
    total_bw: u64,
}

struct AppState {
    contributions: Mutex<HashMap<String, u64>>,
}
