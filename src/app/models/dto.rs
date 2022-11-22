use serde::{Serialize, Deserialize};
#[derive(Deserialize,Serialize)]
pub struct SignupDTO{
    pub first_name:String,
    pub last_name:String,
    pub phone_number:String,
    pub password:String,
}

// #[derive(Deserialize,Serialize)]
// pub enum SigninKind{
//     Password,
//     Code,
// }

#[derive(Deserialize,Serialize)]
pub struct SigninDTO{
    pub phone_number:String,
    pub password:String,
}
#[derive(Deserialize,Serialize,PartialEq, Eq)]
pub enum VerifyKind{
    ChangePassword,
    AccountVerification
}
#[derive(Deserialize,Serialize)]
pub struct VerifyDTO{
    pub phone_number:String,
    pub code:String,
    pub kind:VerifyKind
}

#[derive(Deserialize,Serialize)]
pub struct ForgotPasswordDTO{
    pub phone_number:String,
    pub password:String,
}

#[derive(Deserialize,Serialize)]
pub struct ChangePasswordDTO{
    pub phone_number:String,
    pub password:String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordParams {
    pub sid: String,
}

#[derive(Deserialize,Serialize)]
pub struct SellerSignupDTO{
    pub first_name:String,
    pub last_name:String,
    pub phone_number:String,
    pub password:String,
    pub national_code:String,
    pub national_card_image_sid:String,
    pub bank_card_number:u32,
}

#[derive(Deserialize,Serialize)]
pub struct AddProductDTO{
    pub title:String,
    pub description:String,
    pub price:u32,
    pub discount:Option<u32>,
    pub product_images:Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductParams {
    pub sid: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateProductDTO{
    pub title:String,
    pub description:String,
    pub price:u32,
    pub discount:Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct BuyProductDTO{
    pub address:String,
    pub postal_code:String,
}

#[derive(Deserialize, Serialize)]
pub struct GetProductByUserDTO{
    pub seller_sid:String,
}

#[derive(Deserialize, Serialize)]
pub struct ScheduleOrderDTO{
    pub arrive_at:String,

}