use std::sync::Arc;
use mongodb::sync::Client;
use axum::{response::IntoResponse, extract::{Extension, Path, Multipart}, Json,};
use crate::app::{services::auth, models::dto::{SignupDTO, SigninDTO, VerifyDTO, ForgotPasswordDTO, ChangePasswordParams, ChangePasswordDTO, SellerSignupDTO}};


pub async fn signup(Extension(db):Extension<Arc<Client>>,Extension(redisdb):Extension<Arc<redis::Client>>,Json(data):Json<SignupDTO>)->impl IntoResponse{
    auth::signup(&db, &data,&redisdb).await
}
pub async fn seller_signup(Extension(db):Extension<Arc<Client>>,Extension(redisdb):Extension<Arc<redis::Client>>,Json(data):Json<SellerSignupDTO>)->impl IntoResponse{
    auth::seller_signup(&db, &data,&redisdb).await
}
pub async fn signin(Extension(db):Extension<Arc<Client>>,Extension(redisdb):Extension<Arc<redis::Client>>,Json(data):Json<SigninDTO>)->impl IntoResponse{
    auth::signin(&db, &data,&redisdb).await
}
pub async fn verify(Extension(db):Extension<Arc<Client>>,Extension(redisdb):Extension<Arc<redis::Client>>,Json(data):Json<VerifyDTO>)->impl IntoResponse{
    auth::verify(&db, &data,&redisdb).await
}
pub async fn forgot_password(Extension(db):Extension<Arc<Client>>,Extension(redisdb):Extension<Arc<redis::Client>>,Json(data):Json<ForgotPasswordDTO>)->impl IntoResponse{
    auth::forgot_password(&db, &data,&redisdb).await
}
pub async fn change_password(Extension(db):Extension<Arc<Client>>,Extension(redisdb):Extension<Arc<redis::Client>>,Json(data):Json<ChangePasswordDTO>,Path(params):Path<ChangePasswordParams>)->impl IntoResponse{
    auth::change_password(&db, &data,&params.sid,&redisdb).await
}
pub async fn upload_picture(Extension(db):Extension<Arc<Client>>,multipart:Multipart)->impl IntoResponse{
    auth::upload_picture(&db, multipart).await
}
