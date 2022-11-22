use jsonwebtoken::{Header,encode, EncodingKey, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

use crate::app::models::error::Error;

use super::time::timestamp;

#[derive(Deserialize,Serialize)]
pub struct JWT{
    exp:usize,
    sid:String,
}
const SECRET:&str = "thisisasecret";
impl JWT{
    pub fn new(sid:String,exp:usize)->Self{
        Self{
            sid,
            exp
        }
    }
    pub fn create_token(&self)->Result<String,Error>{
        let header = Header{alg:jsonwebtoken::Algorithm::HS256,kid:Some(String::from("KOO")),..Default::default()};
        match encode(&header, &self, &EncodingKey::from_secret(SECRET.as_bytes())){
            Ok(token)=>Ok(token),
            Err(err)=>Err(Error::CreatingTokenError(err.to_string()))
        }
    }
    pub fn verify_token(token:String)->Result<String,String>{
        match decode::<JWT>(&token, &DecodingKey::from_secret(SECRET.as_bytes()), &Validation::new(jsonwebtoken::Algorithm::HS256)){
            Ok(res)=>{
                if res.claims.exp > timestamp(){
                    Ok(res.claims.sid)
                }else{
                    Err("token expired".to_string())
                }
            },
            Err(_)=>Err("error on decoding token".to_string())
        }
    }
}