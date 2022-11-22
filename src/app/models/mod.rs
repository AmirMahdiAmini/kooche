use serde::{Deserialize, Serialize};

pub mod user;
pub mod dto;
pub mod order;
pub mod error;
pub mod channel;
pub mod product;
#[derive(Deserialize,Serialize)]
pub struct Images{
    pub sid:String,
    pub path:String,
    pub created_at:String
}