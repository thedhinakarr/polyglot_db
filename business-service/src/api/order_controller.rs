use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::order::{CreateOrderDto, UpdateOrderStatusDto};
use crate::services::OrderService;

pub async fn get_all_orders(service: web::Data<OrderService>) -> impl Responder {
    match service.get_all_orders().await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn get_order_by_id(
    service: web::Data<OrderService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    
    match service.get_order(id).await {
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        Ok(None) => HttpResponse::NotFound().json("Order not found"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn create_order(
    service: web::Data<OrderService>,
    order: web::Json<CreateOrderDto>,
) -> impl Responder {
    match service.create_order(order.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn update_order_status(
    service: web::Data<OrderService>,
    path: web::Path<Uuid>,
    status: web::Json<UpdateOrderStatusDto>,
) -> impl Responder {
    let id = path.into_inner();
    
    match service.update_order_status(id, status.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => match e {
            crate::errors::ServiceError::NotFoundError(_) => HttpResponse::NotFound().json(format!("Error: {}", e)),
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}

pub async fn delete_order(
    service: web::Data<OrderService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    
    match service.delete_order(id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            crate::errors::ServiceError::NotFoundError(_) => HttpResponse::NotFound().json(format!("Error: {}", e)),
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}