use serde::{Deserialize, Serialize};

use crate::app::utils::{code::sid_creator, time::formatted_time};

use super::order::Order;

#[derive(Deserialize,Serialize)]
pub struct User{
    pub sid:String,
    pub full_name:String,
    pub phone_number:String,
    pub is_verified:bool,
    pub password:String,
    pub created_at:String,
    pub updated_at:String,
    pub restriction:Restriction,
    pub system_notifications:Vec<String>,
    pub orders:Vec<Order>
}
impl User{
    pub fn new(full_name:String,phone_number:String,password:String)->Self{
        let sid = sid_creator(30);
        let now = formatted_time();
        Self{
            sid,
            full_name,
            phone_number,
            is_verified:false,
            password:password,
            created_at:now.clone(),
            updated_at:now.clone(),
            restriction:Restriction::None,
            system_notifications:vec![format!("ثبت نام در {}",now)],
            orders:Vec::new()
        }
    }
}

#[derive(Deserialize,Serialize)]
pub struct Seller{
    pub sid:String,
    pub full_name:String,
    pub password:String,
    pub national_code:String,
    pub national_card_image_sid:String,
    pub bank_card_number:u32,
    pub phone_number:String,
    pub is_verified:bool,
    pub created_at:String,
    pub updated_at:String,
    pub channel:Option<String>,
    pub status:Status,
    pub system_notifications:Vec<String>,
    pub ongoing_orders:Vec<Order>,
    pub finished_orders:Vec<Order>,
    pub unscheduled_orders:Vec<Order>,
}

impl Seller{
    pub fn new(full_name:String,phone_number:String,password:String,national_code:String,national_card_image_sid:String,bank_card_number:u32)->Self{
        let sid = sid_creator(40);
        let now = formatted_time();
        Self { 
            sid, 
            full_name, 
            password, 
            national_code, 
            national_card_image_sid,
            bank_card_number,
            phone_number,
            is_verified:false, 
            created_at:now.clone(),
            updated_at:now.clone(),
            channel:None,
            status:Status::OnGoing,
            system_notifications:vec![format!("ثبت نام فروشنده در {}",now)],
            ongoing_orders:Vec::new(),
            finished_orders:Vec::new(),
            unscheduled_orders:Vec::new(),
        }
    }
}

#[derive(Deserialize,Serialize,Default)]
pub enum Restriction{
    #[default]
    None,
    Sus,
    Nuisance
}

#[derive(Deserialize,Serialize)]
pub enum Role{
    User,
    Seller,
}
#[derive(Deserialize,Serialize)]
pub enum Status{
    Banned,
    OnGoing
}

#[derive(Deserialize,Serialize)]
pub struct DeletedAccount{
    pub user_sid:String,
    pub phone_number:String,
    pub deleted_at:String,
    pub role:Role,
    pub reason:String,
}