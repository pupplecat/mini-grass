use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::Mutex};

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

fn get_address() -> String {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    format!("{}:{}", host, port)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        contributions: Mutex::new(HashMap::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/report", web::post().to(report_bw))
    })
    .bind(get_address())?
    .run()
    .await
}
