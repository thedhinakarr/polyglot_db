use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use business_service::config::AppConfig;
use business_service::repositories::{PostgresClient, MongoClient, ProductRepository, OrderRepository};
use business_service::services::{ProductService, OrderService};
use business_service::api::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");
    
    // Initialize database clients
    let postgres_client = PostgresClient::new(&config.postgres)
        .await
        .expect("Failed to connect to PostgreSQL");
    
    let mongo_client = MongoClient::new(&config.mongodb)
        .await
        .expect("Failed to connect to MongoDB");
    
    // Initialize repositories
    let product_repository = ProductRepository::new(mongo_client);
    let order_repository = OrderRepository::new(postgres_client);
    
    // Initialize services
    let product_service = web::Data::new(ProductService::new(product_repository));
    let order_service = web::Data::new(OrderService::new(order_repository));
    
    // Start HTTP server
    tracing::info!("Starting server at {}:{}", config.server.host, config.server.port);
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(product_service.clone())
            .app_data(order_service.clone())
            .configure(configure_routes)
    })
    .bind((config.server.host.clone(), config.server.port))?
    .run()
    .await
}