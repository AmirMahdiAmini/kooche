use std::io::Write;

use axum::response::IntoResponse;
use axum::{http::StatusCode, Json, extract::Multipart};
use mongodb::bson;
use crate::app::models::dto::{AddProductDTO, UpdateProductDTO, BuyProductDTO};
use crate::app::models::order::Order;
use crate::app::models::product::Product;
use crate::app::models::user::{Seller, User};
use crate::app::utils::code::sid_creator;
use crate::formatted_time;
use mongodb::options::FindOptions;
pub async fn get_products(db:&mongodb::sync::Client)->impl IntoResponse{
    let mut products = Vec::new();
    match db.database("kooche").collection("products").find(None, FindOptions::builder().limit(20).sort(bson::doc!{"_id":-1}).build()){
        Ok(mut res)=>{
            while let Some(result) = res.next(){
                if result.is_ok(){
                    let product = match bson::from_document::<Product>(result.unwrap()){
                        Ok(p)=>p,
                        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PGP-DOC"}))),
                    };
                    products.push(product);
                }
            }
            return (StatusCode::OK,Json(serde_json::json!({"message":products})))
        }
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PGP-F 2"})))
    }
}
pub async fn get_user_products(db:&mongodb::sync::Client,seller_sid:&String)->impl IntoResponse{
    let mut products = Vec::new();
    match db.database("kooche").collection("products").find(bson::doc!{"seller_sid":seller_sid}, FindOptions::builder().limit(20).sort(bson::doc!{"_id":-1}).build()){
        Ok(mut res)=>{
            while let Some(result) = res.next(){
                if result.is_ok(){
                    let product = match bson::from_document::<Product>(result.unwrap()){
                        Ok(p)=>p,
                        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PGP-DOC"}))),
                    };
                    products.push(product);
                }
            }
            return (StatusCode::OK,Json(serde_json::json!({"message":products})))
        }
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PGP-F 2"})))
    }
}
pub async fn add_product(db:&mongodb::sync::Client,data:&AddProductDTO,user_sid:&String)->impl IntoResponse{
    match db.database("kooche").collection::<Seller>("sellers").find_one(bson::doc!{"sid":user_sid}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
            }
            let product = Product::new(user_sid.to_owned(),data.title.to_owned(), data.description.to_owned(), data.price.to_owned(), data.discount, data.product_images.to_owned());
            let doc = match bson::to_document(&product) {
                Ok(d)=>d,
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PAP-DOC"})))
            };
            match db.database("kooche").collection("products").insert_one(doc, None){
                Ok(_)=>(StatusCode::CREATED,Json(serde_json::json!({"message":"product created"}))),
                Err(_)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PAP-I2"})))
            }
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PAP-F"})))
    }
}
pub async fn get_product_by_sid(db:&mongodb::sync::Client,product_sid:&String)->impl IntoResponse{
    match db.database("kooche").collection::<Product>("products").find_one(bson::doc!{"sid":product_sid}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
            }
            let product = res.unwrap();
            (StatusCode::OK,Json(serde_json::json!({"message":product})))
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PAP-F"})))
    }
}
pub async fn upload_images(db:&mongodb::sync::Client,mut multipart:Multipart)->impl IntoResponse{
    let mut sids = Vec::<String>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "images".to_string(){
        let content_type_item = field.content_type();
        if content_type_item.is_none(){
            return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"unkown content type"})))
        };
        let content_type = content_type_item.unwrap().to_string();
        if content_type== "image/jpeg".to_string(){
            let data = field.bytes().await.unwrap();
            let sid = sid_creator(50);
            let path = format!("uploads/product/{}.jpg",sid);
            match db.database("kooche").collection("images").insert_one(bson::doc!{"sid":sid.clone(),"path":path.clone(),"created_at":formatted_time()}, None){
                Ok(_)=>{
                    let mut file = std::fs::File::create(path).unwrap();
                    file.write(&data[..]).unwrap();
                    sids.push(sid);
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-I"})))
            };
        }
        }
    }
    (StatusCode::OK,Json(serde_json::json!({"message":sids})))
}

pub async fn delete_product(db:&mongodb::sync::Client,product_sid:&String,user_sid:&String)->impl IntoResponse{
    match db.database("kooche").collection::<Seller>("sellers").find_one(bson::doc!{"sid":user_sid}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
            }
            match db.database("kooche").collection::<()>("users").delete_one(bson::doc!{"sid":product_sid}, None){
                Ok(del_res)=>{
                    if del_res.deleted_count == 0{
                        return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"product not found"})))
                    }
                    return (StatusCode::OK,Json(serde_json::json!({"message":"product successfully deleted"})))
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PDP-D"})))
            }
            
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PDP-F"})))
    }
}

pub async fn update_product(db:&mongodb::sync::Client,data:&UpdateProductDTO,product_sid:&String,user_sid:&String)->impl IntoResponse{
    match db.database("kooche").collection::<Seller>("sellers").find_one(bson::doc!{"sid":user_sid}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
            }
            match db.database("kooche").collection::<()>("products").update_one(bson::doc!{"sid":product_sid},
            bson::doc!{
                "title":data.title.to_owned(),
                "description":data.description.to_owned(),
                "price":data.price,
                "discount":data.discount,
            }
            , None){
                Ok(_)=>(StatusCode::CREATED,Json(serde_json::json!({"message":"product created"}))),
                Err(_)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PAP-I"})))
            }
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-F"})))
    }
}
pub async fn buy_product(db:&mongodb::sync::Client,data:&BuyProductDTO,product_sid:&String,user_sid:&String)->impl IntoResponse{
    let seller_sid = match db.database("kooche").collection::<Product>("products").find_one(bson::doc!{"sid":product_sid}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"product not found"})))
            }
            res.unwrap().seller_sid
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-PF"})))
    };
    match db.database("kooche").collection::<Seller>("sellers").find_one(bson::doc!{"sid":seller_sid.clone()}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
            }
            // let reg = Regex::new(r"\b(?!(\d)\1{3})[13-9]{4}[1346-9][013-9]{5}\b").unwrap();
            let order = Order::new(product_sid.to_owned(),user_sid.to_owned(),data.address.to_owned(),data.postal_code.to_owned());
            // if !reg.is_match(&data.postal_code.trim()){
            //     return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"enter the correct postal code"})))
            // };
            let doc = match bson::to_document(&order) {
                Ok(d)=>d,
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PAP-DOC"})))
            };
            match db.database("kooche").collection::<Seller>("sellers").update_one(bson::doc!{"sid":seller_sid},bson::doc!{"$push":{
                "unscheduled_orders":doc.clone()
            }}, None){
                Ok(update_result)=>{
                    if update_result.modified_count == 0{
                        return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-SELLU"})))
                    }
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-UP1"})))
            }
            match db.database("kooche").collection::<Seller>("users").update_one(bson::doc!{"sid":user_sid},bson::doc!{"$push":{
                "orders":doc
            }}, None){
                Ok(update_result)=>{
                    if update_result.modified_count == 0{
                        return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-USU"})))
                    }
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-UP2"})))
            }
            (StatusCode::OK,Json(serde_json::json!({"message":"submited"})))
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PUP-F"})))
    }
}

pub async fn schedule_order(db:&mongodb::sync::Client,data:&String,order_sid:&String,user_sid:&String)->impl IntoResponse{
    println!("SELLER SID BUY PRODUCT {} ORDER SID {}",user_sid,order_sid);

    match db.database("kooche").collection::<Seller>("sellers").find_one(bson::doc!{"sid":user_sid,"unscheduled_orders.sid":order_sid}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
            }
            let orders = res.unwrap().unscheduled_orders;
            let order = orders.iter().find(|n|n.sid == order_sid.to_owned()).unwrap();
            match db.database("kooche").collection::<()>("sellers").update_one(bson::doc!{"sid":user_sid},bson::doc!{
                "$pull":{"unscheduled_orders":{
                    "sid":order_sid
                }},"$push":{
                    "ongoing_orders":bson::doc!{
                        "sid":order.sid.to_owned(),
                        "product_sid":order.product_sid.to_owned(),
                        "user_sid":order.user_sid.to_owned(),
                        "user_address":order.user_address.to_owned(),
                        "postal_code":order.postal_code.to_owned(),
                        "created_at":order.created_at.to_owned(),
                        "arrived_at":data,
                        "finished_at":order.finished_at.to_owned()
                    }
                }
            }, None){
                Ok(result)=>{
                    if result.modified_count == 0{
                        if result.modified_count == 0{
                            return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"order not found"})))
                        }
                    }
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PSD-UPS"})))
            }
            match db.database("kooche").collection::<User>("users").find_one(bson::doc!{"orders.sid":order_sid}, None){
                Ok(result)=>{
                    if result.is_none(){
                        return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
                    };
                    match db.database("kooche").collection::<User>("users").update_one(bson::doc!{"sid":result.unwrap().sid,"orders.sid":order_sid},
                bson::doc!{"$set":{"orders.$.arrived_at":data}}, None){
                        Ok(result)=>{
                                if result.modified_count == 0{
                                    return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"order not found"})))
                                }
                            },
                        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PSD-F3-1"}))),
                    };
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PSD-F3"}))),
            };
            
            (StatusCode::OK,Json(serde_json::json!({"message":"order scheduled"})))
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#PSD-F1"})))
    }    
}

pub async fn finished_order(db:&mongodb::sync::Client,order_sid:&String,user_sid:&String)->impl IntoResponse{
    match db.database("kooche").collection::<User>("users").find_one(bson::doc!{"sid":user_sid,"scheduled_orders.sid":order_sid}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
            }
            let orders = res.unwrap().orders;
            let order = orders.iter().find(|n|n.sid == order_sid.to_owned()).unwrap();
            match db.database("kooche").collection::<()>("sellers").update_one(bson::doc!{"sid":user_sid},bson::doc!{
                "$pull":{"scheduled_orders":{
                    "sid":order_sid
                }},"$push":{
                    "finished_orders":bson::doc!{
                        "sid":order.sid.to_owned(),
                        "product_sid":order.product_sid.to_owned(),
                        "user_sid":order.user_sid.to_owned(),
                        "user_address":order.user_address.to_owned(),
                        "postal_code":order.postal_code.to_owned(),
                        "created_at":order.created_at.to_owned(),
                        "arrived_at":order.arrived_at.to_owned(),
                        "finished_at":formatted_time(),
                    }
                }
            }, None){
                Ok(result)=>{
                    if result.modified_count == 0{
                        if result.modified_count == 0{
                            return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"order not found"})))
                        }
                    }
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#FO-UPS"})))
            }
            match db.database("kooche").collection::<()>("users").update_one(bson::doc!{"sid":user_sid},bson::doc!{
                "$pull":{"orders":{
                    "sid":order_sid
                }}
            }, None){
                Ok(result)=>{
                    if result.modified_count == 0{
                        if result.modified_count == 0{
                            return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"order not found"})))
                        }
                    }
                },
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#FO-UPU"})))
            }
            (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"order successfully arrived"})))
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#FO-F"})))
    }
} 
