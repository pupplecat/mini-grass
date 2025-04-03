use std::{io::Result, sync::Arc};

use actix_web::{
    dev::{HttpServiceFactory, Server},
    web, App, HttpServer,
};
use tracing::info;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::{container::ServiceContext, core::config::Config, transports::actix::health_handler};

use super::report_bandwidth_handler;

pub async fn serve(config: &Config) -> Result<()> {
    initial_logger();

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
        let app = App::new().app_data(web::Data::from(ctx.clone()));

        app.service(build_api()).service(health_handler::health)
    })
    .bind(address)?;

    info!("binding address: {}", address);

    Ok(server.run())
}

fn build_api() -> impl HttpServiceFactory {
    web::scope("/api")
        .service(web::scope("/report").service(report_bandwidth_handler::report_bandwidth))
}

pub fn initial_logger() {
    let env_filter_layer = EnvFilter::new("INFO");

    let _ = LogTracer::init();
    let formatter_layer = tracing_subscriber::fmt::layer().with_test_writer();
    let subscriber = Registry::default()
        .with(env_filter_layer)
        .with(JsonStorageLayer)
        .with(formatter_layer);

    // panic here is expected, in e2e, subscriber my already been set by prior test case.
    let _ = tracing::subscriber::set_global_default(subscriber);
}
