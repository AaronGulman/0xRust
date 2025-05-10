use dotenv::dotenv;
use std::env;

pub fn init(){
    dotenv().ok();
    
    let db_url = env::var("DB_URL").expect("DB URL NOT FOUND");
    println!("{:?}",db_url)
}
