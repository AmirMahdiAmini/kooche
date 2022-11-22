use std::time::{SystemTime, UNIX_EPOCH};

pub fn formatted_time()->String{
    chrono::Local::now().format("%H:%M:%S %Y/%m/%d").to_string()
}
pub fn timestamp()->usize{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize
}