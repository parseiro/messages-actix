extern crate dotenv;

use dotenv::dotenv;
use std::env;
use messages_actix::Blog;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let bl = Blog::new(8998);
    bl.run(database_url)
}
