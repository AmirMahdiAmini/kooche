use std::{fs::OpenOptions, io::Write};
pub mod app;
pub use app::utils::time::formatted_time;

pub fn get_env(key:&str)->String{
    std::env::var(key).expect(format!("NOT FOUND ENV {}",key).as_str())
}

#[inline]
pub fn write_log(message:String){
    match OpenOptions::new().append(true).write(true).open("./src/logs/log.log") {
        Ok(mut file) => {
            file.write_all(message.as_bytes()).unwrap();
        },
        Err(_) => {},
    };
}
#[macro_export]
macro_rules! logger {
    ( $level:expr, $e:expr ) => {
        match $level {
            "INFO" => {
                $crate::write_log(format!("[INFO] | {} | {} \n",$e,$crate::formatted_time()));
                println!("\x1b[44;97m[KooCHe] {}  {}\x1b[0m",$e,$crate::formatted_time());
            },
            "WARN" =>{
                $crate::write_log(format!("[WARN] | {} | {} \n",$e,$crate::formatted_time()));
                println!("\x1b[43;97m[KooCHe] {}  {}\x1b[0m",$e,$crate::formatted_time());
            },
            "ERROR" => {
                $crate::write_log(format!("[ERROR] | {} | {} \n",$e,$crate::formatted_time()));
                println!("\x1b[41;97m[KooCHe] {}  {}\x1b[0m",$e,$crate::formatted_time());
            },
            "MESSAGE" =>{
                $crate::write_log(format!("[MESSAGE] | {} | {} \n",$e,$crate::formatted_time()));
                println!("\x1b[42;97m[KooCHe] {}  {}\x1b[0m",$e,$crate::formatted_time());
            },
            _ => {
                $crate::write_log(format!("[MESSAGE] | {} | {} \n",$e,$crate::formatted_time()));
                println!("\x1b[42;97m[KooCHe] {}  {}\x1b[0m",$e,$crate::formatted_time());
            },
        }
    };
} 