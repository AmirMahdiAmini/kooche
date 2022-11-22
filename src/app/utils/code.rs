use rand::{thread_rng, Rng, distributions::Alphanumeric};

pub fn sid_creator(len:u8)->String{
    thread_rng().sample_iter(Alphanumeric).take(len as usize).map(char::from).collect::<String>()
}
pub fn verify_creator()->u32{
    let code:u32 = rand::thread_rng().gen_range(132568..987984);
    code
}

pub fn seller_verify_creator()->u16{
    let code:u16 = rand::thread_rng().gen_range(13256..65535);
    code
}