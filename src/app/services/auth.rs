use std::io::Write;
use axum::{response::IntoResponse, http::StatusCode, Json, extract::Multipart};
use mongodb::bson;
use redis::Commands;
use crate::app::models::{dto::{SignupDTO, SigninDTO, VerifyDTO, VerifyKind, ForgotPasswordDTO, ChangePasswordDTO, SellerSignupDTO}, user::{User, Seller}, Images};
use crate::app::utils::{jwt::JWT, time::{timestamp,formatted_time}, code::sid_creator};
use crate::app::pkg::is_valid_iranian_national_code;
use super::sms::SMSAuthentication;

pub async fn signup(db:&mongodb::sync::Client,data:&SignupDTO,redisdb:&redis::Client)->impl IntoResponse{
    match db.database("kooche").collection::<Seller>("sellers").find_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
        Ok(u)=>{
                if u.is_some(){
                    return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"user already exists"})))
                }
            },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SU-CH1"})))
    };
    match db.database("kooche").collection("users").find_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
        Ok(u)=>{
            if u.is_some(){
                    let user = match bson::from_document::<User>(u.unwrap()) {
                        Ok(d)=>d,
                        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SU-DOC1"})))
                    };
                    if !user.is_verified{
                        match SMSAuthentication::verify_sms(&user.phone_number, super::sms::SMSKind::Signup,redisdb.clone()).await{
                            Ok(_)=>return (StatusCode::OK,Json(serde_json::json!({"message":"verify your account"}))),
                            Err(err)=>return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
                        }
                    }
                    return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"user already exists"})))
                }
            },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SU-CH"})))
    };
    let password = match bcrypt::hash(&data.password, bcrypt::DEFAULT_COST){
        Ok(p)=>p,
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SU-P"})))
    };
    let user = User::new(format!("{} {}",data.first_name,data.last_name), data.phone_number.to_owned(), password);
    let doc = match bson::to_document(&user) {
        Ok(d)=>d,
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SU-DOC2"})))
    };
    match SMSAuthentication::verify_sms(&user.phone_number, super::sms::SMSKind::Signup,redisdb.to_owned()).await{
        Ok(_)=>(),
        Err(err)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":err})))
    }
    match db.database("kooche").collection("users").insert_one(doc, None){
        Ok(_)=>(StatusCode::CREATED,Json(serde_json::json!({"message":"account created, now verify it"}))),
        Err(_)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SU-D"})))
    }
}
pub async fn seller_signup(db:&mongodb::sync::Client,data:&SellerSignupDTO,redisdb:&redis::Client)->impl IntoResponse{
    match db.database("kooche").collection("sellers").find_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
        Ok(u)=>{
            if u.is_some(){
                    let user = match bson::from_document::<Seller>(u.unwrap()) {
                        Ok(d)=>d,
                        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSU-DOC1"})))
                    };
                    if !user.is_verified{
                        match SMSAuthentication::verify_sms(&user.phone_number, super::sms::SMSKind::Signup,redisdb.clone()).await{
                            Ok(_)=>return (StatusCode::OK,Json(serde_json::json!({"message":"verify your account"}))),
                            Err(err)=>return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
                        }
                    }
                    match db.database("kooche").collection::<Seller>("sellers").find_one(bson::doc! {"national_code":data.national_code.clone()}, None){
                        Ok(res)=>{
                            if res.is_some(){
                                return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"user already exists"})))    
                            }
                        },
                        Err(e)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":e.to_string()})))
                    }
                    return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"user already exists"})))
                }
            },
            
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSU-CH"})))
    };
    match db.database("kooche").collection::<User>("users").delete_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
        Ok(_)=>(),
        Err(e)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":e.to_string()})))
    }
    match db.database("kooche").collection::<Images>("images").find_one(bson::doc!{"sid":data.national_card_image_sid.clone()}, None){
        Ok(res)=>{
            if res.is_none(){
                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"image not found"}))) 
            }
        },
        Err(e)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":e.to_string()})))
    }
    if !is_valid_iranian_national_code(&data.national_code){
        return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"make sure you send the correct information"})))
    };
    let password = match bcrypt::hash(&data.password, bcrypt::DEFAULT_COST){
        Ok(p)=>p,
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSU-P"})))
    };
    let seller = Seller::new(format!("{} {}",data.first_name,data.last_name),data.phone_number.to_owned(),password,data.national_code.to_owned(),data.national_card_image_sid.to_owned(),data.bank_card_number.to_owned());
    let doc = match bson::to_document(&seller) {
        Ok(d)=>d,
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSU-DOC2"})))
    };
    match SMSAuthentication::verify_sms(&seller.phone_number, super::sms::SMSKind::SellerSignup,redisdb.to_owned()).await{
        Ok(_)=>(),
        Err(err)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":err})))
    }
    match db.database("kooche").collection("sellers").insert_one(doc, None){
        Ok(_)=>(StatusCode::CREATED,Json(serde_json::json!({"message":"seller account created, now verify it"}))),
        Err(_)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSU-D"})))
    }
}
pub async fn signin(db:&mongodb::sync::Client,data:&SigninDTO,redisdb:&redis::Client)->impl IntoResponse{
    match db.database("kooche").collection("users").find_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
        Ok(res)=>{
            if res.is_none(){
                match db.database("kooche").collection("sellers").find_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
                    Ok(res)=>{
                        if res.is_none(){
                            return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
                        }
                        let seller =match bson::from_document::<Seller>(res.unwrap()) {
                            Ok(u)=>u,
                            Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSI-SEC1DOC"})))
                        };
                        if !seller.is_verified{
                            match SMSAuthentication::verify_sms(&seller.phone_number, super::sms::SMSKind::SellerSignup,redisdb.clone()).await{
                                Ok(_)=>return (StatusCode::OK,Json(serde_json::json!({"message":"verify your account"}))),
                                Err(err)=>return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
                            }
                        }
                        match bcrypt::verify(data.password.clone(), seller.password.as_str()){
                            Ok(p)=>{
                                if p {
                                    let token = JWT::new(seller.sid, timestamp() + 600).create_token();
                                    if token.is_err(){
                                        return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSI-SEC1T"})))
                                    }
                                    return (StatusCode::OK,Json(serde_json::json!({"message":token.unwrap()})))
                                }else{
                                    return  (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"password or phone number is incorrect"})))
                                }
                            }
                            Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSI-SEC1P"})))
                        }
                    },
                    Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SSI-SEC1F1"})))
                }
            }
            let user =match bson::from_document::<User>(res.unwrap()) {
                Ok(u)=>u,
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SI-DOC"})))
            };
            if !user.is_verified{
                match SMSAuthentication::verify_sms(&user.phone_number, super::sms::SMSKind::Signup,redisdb.clone()).await{
                    Ok(_)=>return (StatusCode::OK,Json(serde_json::json!({"message":"verify your account"}))),
                    Err(err)=>return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
                }
            }
            match bcrypt::verify(data.password.clone(), user.password.as_str()){
                Ok(p)=>{
                    if p {
                        let token = JWT::new(user.sid, timestamp() + 900).create_token();
                        if token.is_err(){
                            return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SI-T"})))
                        }
                        (StatusCode::OK,Json(serde_json::json!({"message":token.unwrap()})))
                    }else{
                        (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"password or phone number is incorrect"})))
                    }
                }
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SI-P"})))
            }
        },
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SI-F1"})))
    }
}
pub async fn verify(db:&mongodb::sync::Client,data:&VerifyDTO,redisdb:&redis::Client)->impl IntoResponse{
    let mut redisdb = redisdb.to_owned();
    let _:String = match redisdb.get(data.phone_number.clone()){
        Ok(code)=>{
            if code != data.code.clone(){
                return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"wrong"})))
            }
            code
        },
        Err(_)=>return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"not found"})))
    };
    let _:() = match redisdb.del(data.phone_number.clone()){
        Ok(r)=>r,
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#VF-RDD"})))
    };
    if data.kind == VerifyKind::AccountVerification{
        match db.database("kooche").collection::<()>("users").update_one(bson::doc!{"phone_number":data.phone_number.clone()},bson::doc!{"is_verified":true}, None){
            Ok(res)=>{
                if res.matched_count == 0{
                    match db.database("kooche").collection::<()>("sellers").update_one(bson::doc!{"phone_number":data.phone_number.clone()},bson::doc!{"is_verified":true}, None){
                        Ok(res)=>{
                            if res.matched_count == 0{
                                return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
                            }
                            if res.modified_count == 0{
                                return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#VF-MC"})))
                            }
                            return (StatusCode::OK,Json(serde_json::json!({"message":"your account successfully verified"})))
                        },
                        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#VF-UP"})))
                    }
                }
                if res.modified_count == 0{
                    return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#VF-MC"})))
                }
                (StatusCode::OK,Json(serde_json::json!({"message":"your account successfully verified"})))
            },
            Err(_)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#VF-UP"})))
        }
    }else{
        let link = sid_creator(55);
        let _:() = match redisdb.set_ex(data.phone_number.clone(),link.clone(),120){
            Ok(d)=>d,
            Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#VF-RDS"})))
        };
        (StatusCode::OK,Json(serde_json::json!({"message":link})))
    }
}
pub async fn forgot_password(db:&mongodb::sync::Client,data:&ForgotPasswordDTO,redisdb:&redis::Client)->impl IntoResponse{
    match db.database("kooche").collection("users").find_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
        Ok(res)=>{
            if res.is_none(){
                match db.database("kooche").collection("users").find_one(bson::doc!{"phone_number":data.phone_number.clone()}, None){
                    Ok(res)=>{
                        if res.is_none(){
                            return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found"})))
                        }
                        let user =match bson::from_document::<User>(res.unwrap()) {
                            Ok(u)=>u,
                            Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SI-DOC"})))
                        };
                        if !user.is_verified{
                            match SMSAuthentication::verify_sms(&user.phone_number.clone(), super::sms::SMSKind::SellerSignup,redisdb.clone()).await{
                                Ok(_)=>return (StatusCode::OK,Json(serde_json::json!({"message":"verify your account"}))),
                                Err(err)=>return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
                            }
                        }
                        match SMSAuthentication::verify_sms(&user.phone_number.clone(), super::sms::SMSKind::ForgotPassword,redisdb.clone()).await{
                            Ok(_)=>return (StatusCode::OK,Json(serde_json::json!({"message":"code is sent, verify it"}))),
                            Err(err)=>return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
                        }
                    }
                    Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#FP-FT"})))
                }
            }
            let user =match bson::from_document::<User>(res.unwrap()) {
                Ok(u)=>u,
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#SI-DOC"})))
            };
            if !user.is_verified{
                match SMSAuthentication::verify_sms(&user.phone_number.clone(), super::sms::SMSKind::Signup,redisdb.clone()).await{
                    Ok(_)=>return (StatusCode::OK,Json(serde_json::json!({"message":"verify your account"}))),
                    Err(err)=>return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
                }
            }
            match SMSAuthentication::verify_sms(&user.phone_number.clone(), super::sms::SMSKind::ForgotPassword,redisdb.clone()).await{
                Ok(_)=>(StatusCode::OK,Json(serde_json::json!({"message":"code is sent, verify it"}))),
                Err(err)=>(StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":err})))
            }
        },
        Err(_)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#FP-F"})))
    }
}
pub async fn change_password(db:&mongodb::sync::Client,data:&ChangePasswordDTO,sid:&String,redisdb:&redis::Client)->impl IntoResponse{
    if sid.is_empty(){
        return (StatusCode::UNAUTHORIZED,Json(serde_json::json!({"error":"UNAUTHORIZED"})))
    };
    let mut redisdb = redisdb.to_owned();
    let _:String = match redisdb.get(data.phone_number.clone()){
        Ok(sid_data)=>{
            if &sid_data != sid{
                return (StatusCode::UNAUTHORIZED,Json(serde_json::json!({"error":"unauthorized"})))
            }
            sid_data
        },
        Err(_)=>return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"unkown sid"})))
    };
    let password = match bcrypt::hash(&data.password, bcrypt::DEFAULT_COST){
        Ok(p)=>p,
        Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#CP-P"})))
    };
    match db.database("kooche").collection::<()>("users").update_one(bson::doc!{"phone_number":data.phone_number.clone()}, bson::doc!{"password":password.clone()}, None){
        Ok(res)=>{
            if res.matched_count == 0{
                match db.database("kooche").collection::<()>("sellers").update_one(bson::doc!{"phone_number":data.phone_number.clone()}, bson::doc!{"password":password}, None){
                    Ok(res)=>{
                        if res.matched_count == 0{
                            return (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"user not found #E1"})))
                        }
                        if res.modified_count == 0{
                            return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"CP-MC"})))
                        }
                        return (StatusCode::OK,Json(serde_json::json!({"message":"your password successfully changed"})))
                    },
                    Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#CP-UP"})))
                }
            }
            if res.modified_count == 0{
                return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"CP-MC"})))
            }
            (StatusCode::OK,Json(serde_json::json!({"message":"your password successfully changed"})))
        },
        Err(_)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#CP-UP"})))
    }
}
pub async fn upload_picture(db:&mongodb::sync::Client,mut multipart:Multipart)->impl IntoResponse{
    while let Ok(field) = multipart.next_field().await {
        if field.is_none(){
            return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"#UP-NONE"})))
        }
        let field = field.unwrap();
        let name = field.name().unwrap().to_string();
        if name == "file".to_string(){
        let content_type_item = field.content_type();
        if content_type_item.is_none(){
            return (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":"unkown content type"})))
        };
        let content_type = content_type_item.unwrap().to_string();
        if content_type== "image/jpeg".to_string(){
            let data = field.bytes().await.unwrap();
            let sid = sid_creator(45);
            let path = format!("uploads/auth/{}.jpg",sid);
            match db.database("kooche").collection("images").insert_one(bson::doc!{"sid":sid.clone(),"path":path.clone(),"created_at":formatted_time()}, None){
                Ok(_)=>{
                    let mut file = std::fs::File::create(path).unwrap();
                    file.write(&data[..]).unwrap();
                return (StatusCode::OK,Json(serde_json::json!({"message":sid})))},
                Err(_)=>return (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#AUP-I"})))
            };
        }
        }
    }
    (StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":"#AUP-WH"})))
}