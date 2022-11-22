use serde::{Serialize, Deserialize};
use crate::{formatted_time, app::utils::code::sid_creator};

#[derive(Deserialize,Serialize)]
pub struct Channel{
    pub sid:String,
    pub channel_address:String,
    pub creator_sid:String,
}

#[derive(Deserialize,Serialize)]
pub struct Post{
    pub sid:String,
    pub channel_sid:String,
    pub content:Option<String>,
    pub images:Option<Vec<String>>,
    pub created_at:String,
    pub edited:bool,
}

impl Post{
    pub fn new(content:Option<String>,images:Option<Vec<String>>,channel_sid:String,)->Self{
        let sid = sid_creator(35);
        Self{
            sid,
            channel_sid,
            content,
            images,
            created_at:formatted_time(),
            edited:false,
        }
    }
}

#[derive(Deserialize,Serialize,Default)]
pub enum Restriction{
    #[default]
    None,
    Offensive,
    Scam,
    Destructive,
}