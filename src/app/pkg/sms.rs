use reqwest;
use serde::{Serialize, Deserialize};
use crate::get_env;

#[derive(Serialize, Debug)]
pub struct Message {
    #[serde(rename = "messageText")]
    pub message_text: String,
    pub mobiles: Vec<String>,
    #[serde(rename = "lineNumber")]
    pub line_number: String,
}
#[derive(Deserialize, Debug)]
struct SendMessageResponse {
    status: u8,
}
impl Message {
    pub fn new(message: String, mobile_number: String) -> Self {
        Self {
            message_text: message,
            mobiles: vec![mobile_number],
            line_number: get_env("SMS_LINE_NUMBER"),
        }
    }
    pub async fn send_message(&self) -> Result<(), String> {
        let client = reqwest::Client::new();
        let response = client
        .post("https://api.sms.ir/v1/send/bulk")
        .json(&self)
        .header("X-API-KEY", get_env("SMS_SECRET_KEY"))
        .send()
        .await;
        let response = match response {
            Ok(resp) => resp,
            Err(_) => return Err(String::from("failed to send message!")),
        };
        let result = response.json::<SendMessageResponse>().await;
        let result = match result {
            Ok(re) => re,
            Err(_) =>
            {
                return Err(String::from("failed to send message!"));
            }
        };
        match result.status {
            1 => Ok(()),        
            0 => {
                return Err(String::from("failed to send message!"));
           }
           _ => {
                return Err(String::from("failed to send message!"));
           }
        }
    }
}
