//use messages_actix::MessageApp;

use dotenv::dotenv;
use std::env;
//use std::io::Result;

fn main() -> std::io::Result<()> {    
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Blog::new(8998)
        .run(database_url)
}

