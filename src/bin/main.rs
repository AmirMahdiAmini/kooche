use std::{net::SocketAddr, sync::Arc};
use axum::{Router, routing::{get, post,delete}, response::IntoResponse, http::StatusCode, Json, Extension, handler::Handler, middleware};
use koochelib::app::controllers::product::{get_product, get_product_by_sid, get_user_product, buy_product, finished_order};
use koochelib::app::middleware::auth_middleware;
use koochelib::app::database::{mongodb_connection, redisdb_connection};
use koochelib::app::controllers::auth::{signup,seller_signup, signin, verify, forgot_password, change_password,upload_picture};
use koochelib::app::controllers::product::{upload_images,add_product,delete_product,schedule_order};
use koochelib::logger;
use tower_http::limit::RequestBodyLimitLayer;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    logger!("INFO","Redis Connecting...");
    let redisdb =  redisdb_connection().unwrap();
    logger!("INFO","Mongo Connecting...");
    let mongodb = mongodb_connection().unwrap();
    
    let app = Router::new()
    .route("/product/upload", post(upload_images))
    .route("/product/add", post(add_product))
    .route("/product/delete/:sid", delete(delete_product))
    .route("/product", get(get_product))
    .route("/product/seller", get(get_user_product))
    .route("/product/:sid", get(get_product_by_sid))
    .route("/product/buy/:sid", post(buy_product))
    .route("/product/order/schedule/:sid", post(schedule_order))
    .route("/product/order/:sid", get(finished_order))
    .route_layer(middleware::from_fn(auth_middleware))
    .route("/", get(index))
    .route("/signup", post(signup))
    .route("/seller/signup", post(seller_signup))
    .route("/signin", post(signin))
    .route("/verify", post(verify))
    .route("/forgot_password", post(forgot_password))
    .route("/change_password/:sid", post(change_password))
    .route("/auth/upload", post(upload_picture))
    .layer(Extension(Arc::new(redisdb)))
    .layer(Extension(Arc::new(mongodb)))
    .layer(RequestBodyLimitLayer::new( 
        5 * 1024 * 1024, /* 5mb */
    ))
    ;

    let app = app.fallback(not_found.into_service());

    let addr = SocketAddr::from(([127,0,0,1],8080));
    logger!("INFO","Server Starting...");
    axum::Server::bind(&addr).serve(app.into_make_service())
    .await
    .expect("Server Error");


    Ok(())
}
async fn index()->String{
    format!("hello")
}
async fn not_found()->impl IntoResponse{
    (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"not found"})))
}