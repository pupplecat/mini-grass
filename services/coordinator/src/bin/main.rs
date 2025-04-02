use coordinator::core::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::from_env();
    coordinator::transports::actix::server::serve(&config).await
}
