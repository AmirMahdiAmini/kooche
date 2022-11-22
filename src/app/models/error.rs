use std::fmt::Display;

use serde::{Serialize, Deserialize};


#[derive(Debug,Serialize,Deserialize)]
pub enum Error{
    CreatingTokenError(String),
    VerificationSMS(String),

}
impl std::error::Error for Error{}

impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)  
    }
}

// impl ToString for Error{
//     fn to_string(&self) -> String {
//         match self{
//             Self::CreatingTokenError(s)=>format!("{s}"),
//             Self::VerificationSMS(s)=>format!("{s}"),
//         }
//     }
// }