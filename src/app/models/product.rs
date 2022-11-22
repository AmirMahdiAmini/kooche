use serde::{Deserialize, Serialize};

use crate::{app::utils::code::sid_creator, formatted_time};

#[derive(Deserialize,Serialize)]
pub struct Product{
    pub sid:String,
    pub seller_sid:String,
    pub title:String,
    pub description:String,
    pub price:u32,
    pub discount:Option<u32>,
    pub product_images:Option<Vec<String>>,
    pub created_at:String,
    pub updated_at:String,
}

impl Product{
    pub fn new(seller_sid:String,title:String,description:String,price:u32,discount:Option<u32>,product_images:Option<Vec<String>>)->Self{
        let sid = sid_creator(40);
        let now = formatted_time();
        Self{
            sid,
            seller_sid,
            title,
            description,
            price,
            discount,
            product_images,
            created_at:now.clone(),
            updated_at:now.clone()
        }
    }
}

