use axum::{
    http::{Request, StatusCode},
    response::Response,
    middleware::Next,
    http,
};

use super::utils::jwt::JWT;

#[derive(Clone)]
pub struct MiddlewareData{
    pub sid:String,
}
pub async fn auth_middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
        let auth_header = if let Some(auth_header) = auth_header {
            auth_header
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        };
        
    if let Ok(sid) = JWT::verify_token(auth_header.split_once(" ").unwrap().1.to_string()) {
        println!("SID {}",sid);
        let data = MiddlewareData{
            sid
        };
        req.extensions_mut().insert(data);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
