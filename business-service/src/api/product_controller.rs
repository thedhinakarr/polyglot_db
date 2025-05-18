use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::product::{CreateProductDto, UpdateProductDto};
use crate::services::ProductService;

pub async fn get_all_products(service: web::Data<ProductService>) -> impl Responder {
    match service.get_all_products().await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn get_product_by_id(
    service: web::Data<ProductService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    
    match service.get_product(id).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().json("Product not found"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn create_product(
    service: web::Data<ProductService>,
    product: web::Json<CreateProductDto>,
) -> impl Responder {
    match service.create_product(product.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn update_product(
    service: web::Data<ProductService>,
    path: web::Path<Uuid>,
    product: web::Json<UpdateProductDto>,
) -> impl Responder {
    let id = path.into_inner();
    
    match service.update_product(id, product.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => match e {
            crate::errors::ServiceError::NotFoundError(_) => HttpResponse::NotFound().json(format!("Error: {}", e)),
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}

pub async fn delete_product(
    service: web::Data<ProductService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    
    match service.delete_product(id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            crate::errors::ServiceError::NotFoundError(_) => HttpResponse::NotFound().json(format!("Error: {}", e)),
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}