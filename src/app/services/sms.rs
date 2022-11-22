use redis::Commands;

use crate::app::utils::code::{verify_creator, seller_verify_creator};
use crate::app::pkg::sms::Message;


#[derive(PartialEq, Eq)]
pub enum SMSKind{
    Signup,
    ForgotPassword,
    SellerSignup,
    Signin
}

impl ToString for SMSKind{
    fn to_string(&self) -> String {
        match self{
            SMSKind::Signup=>String::from("ثبت نام"),
            SMSKind::SellerSignup=>String::from(" ثبت نام فروشنده"),
            SMSKind::ForgotPassword=>String::from("تغییر رمز"),
            SMSKind::Signin=>String::from("ورورد به حساب"),
        }
    }
}

pub struct SMSAuthentication;

impl SMSAuthentication{
    pub async fn verify_sms(phone:&String,message:SMSKind,mut redisdb:redis::Client)->Result<(),String>{
        let c:String = match redisdb.get(&phone.clone()){
            Ok(d)=>d,
            Err(_)=>{
                String::from("n115")
            }
        };
        if c == "n115"{
        }else{
            return Err(String::from("کد به تازگی ارسال شده است"))
        }

        let code:usize; 
        if message == SMSKind::SellerSignup{
            code =  seller_verify_creator() as usize
        }else{
            code = verify_creator() as usize
        }
        let _:() = match redisdb.set_ex(phone.clone(),code.clone(),120){
            Ok(d)=>d,
            Err(_)=>return Err(String::from("چند دقیقه دیگر مجدد امتحان کنید"))
        };
        match Message::new(format!("{} \n کد تایید: {}",message.to_string(),code.clone()), String::from(phone.clone())).send_message().await{
            Ok(_)=> Ok(()),
            Err(_)=> Err(String::from("خطا در ارسال پیامک"))
        }
    }
}