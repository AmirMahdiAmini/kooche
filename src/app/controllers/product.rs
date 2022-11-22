use std::sync::Arc;
use mongodb::sync::Client;
use axum::{response::IntoResponse, extract::{Extension, Multipart, Path}, Json};

use crate::app::{models::dto::{AddProductDTO, ProductParams, UpdateProductDTO, BuyProductDTO, GetProductByUserDTO, ScheduleOrderDTO}, middleware::MiddlewareData};
use crate::app::services::product;

pub async fn get_product(Extension(db):Extension<Arc<Client>>)->impl IntoResponse{
    product::get_products(&db).await
}
pub async fn get_user_product(Extension(db):Extension<Arc<Client>>,Json(data):Json<GetProductByUserDTO>)->impl IntoResponse{
    product::get_user_products(&db,&data.seller_sid).await
}
pub async fn get_product_by_sid(Extension(db):Extension<Arc<Client>>,Path(params):Path<ProductParams>)->impl IntoResponse{
    product::get_product_by_sid(&db,  &params.sid).await
}
pub async fn add_product(Extension(db):Extension<Arc<Client>>,Extension(user_sid): Extension<MiddlewareData>,Json(data):Json<AddProductDTO>)->impl IntoResponse{
    product::add_product(&db, &data,&user_sid.sid).await
}
pub async fn upload_images(Extension(db):Extension<Arc<Client>>,multipart:Multipart)->impl IntoResponse{
    product::upload_images(&db,multipart).await
}
pub async fn delete_product(Extension(db):Extension<Arc<Client>>,Extension(user_sid): Extension<MiddlewareData>,Path(params):Path<ProductParams>)->impl IntoResponse{
    product::delete_product(&db, &params.sid, &user_sid.sid).await
}
pub async fn update_product(Extension(db):Extension<Arc<Client>>,Extension(user_sid): Extension<MiddlewareData>,Json(data):Json<UpdateProductDTO>,Path(params):Path<ProductParams>)->impl IntoResponse{
    product::update_product(&db, &data, &params.sid, &user_sid.sid).await
}
pub async fn buy_product(Extension(db):Extension<Arc<Client>>,Json(data):Json<BuyProductDTO>,Extension(user_sid): Extension<MiddlewareData>,Path(params):Path<ProductParams>)-> impl IntoResponse{
    product::buy_product(&db, &data, &params.sid, &user_sid.sid).await
}
pub async fn schedule_order(Extension(db):Extension<Arc<Client>>,Json(data):Json<ScheduleOrderDTO>,Extension(user_sid): Extension<MiddlewareData>,Path(params):Path<ProductParams>)-> impl IntoResponse{
    product::schedule_order(&db, &data.arrive_at, &params.sid, &user_sid.sid).await
}
pub async fn finished_order(Extension(db):Extension<Arc<Client>>,Extension(user_sid): Extension<MiddlewareData>,Path(params):Path<ProductParams>)-> impl IntoResponse{
    product::finished_order(&db, &params.sid, &user_sid.sid).await
}