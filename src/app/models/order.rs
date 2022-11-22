use serde::{Deserialize, Serialize};

use crate::{app::utils::code::sid_creator, formatted_time};

#[derive(Debug,Deserialize,Serialize)]
pub struct Order{
    pub sid:String,
    pub product_sid:String,
    pub user_sid:String,
    pub user_address:String,
    pub postal_code:String,
    pub created_at:String,
    pub arrived_at:Option<String>,
    pub finished_at:Option<String>,
}
impl Order{
    pub fn new(product_sid:String,user_sid:String,user_address:String,postal_code:String,)->Self{
        let sid = sid_creator(25);
        Self { 
            sid,
            product_sid, 
            user_sid, 
            user_address, 
            postal_code, 
            created_at:formatted_time(), 
            arrived_at: None, 
            finished_at: None, 
        }
    }
}