use actix_web::web;
use crate::api::{
    product_controller, 
    order_controller
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Product routes
    cfg.service(
        web::scope("/api/products")
            .route("", web::get().to(product_controller::get_all_products))
            .route("", web::post().to(product_controller::create_product))
            .route("/{id}", web::get().to(product_controller::get_product_by_id))
            .route("/{id}", web::put().to(product_controller::update_product))
            .route("/{id}", web::delete().to(product_controller::delete_product))
    );
    
    // Order routes
    cfg.service(
        web::scope("/api/orders")
            .route("", web::get().to(order_controller::get_all_orders))
            .route("", web::post().to(order_controller::create_order))
            .route("/{id}", web::get().to(order_controller::get_order_by_id))
            .route("/{id}/status", web::patch().to(order_controller::update_order_status))
            .route("/{id}", web::delete().to(order_controller::delete_order))
    );
    
    // Health check
    cfg.route("/health", web::get().to(health_check));
}

async fn health_check() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "up",
        "message": "Business service is running"
    }))
}