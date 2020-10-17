#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![warn(anonymous_parameters)]
//#![warn(missing_docs)]
// #![warn(trivial_casts, trivial_numeric_casts)]
// #![warn(unused_results)]
// #![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![warn(clippy::cast_possible_truncation,clippy::cast_possible_wrap,
clippy::cast_precision_loss,clippy::cast_sign_loss,clippy::integer_arithmetic)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filter_map,clippy::filter_map_next)]
#![warn(clippy::if_not_else,clippy::nonminimal_bool,clippy::single_match_else)]
#![warn(clippy::int_plus_one)]
#![warn(clippy::similar_names)]
#![warn(clippy::mutex_integer)]
//#![warn(clippy::print_stdout,clippy::use_debug)]
#![warn(clippy::unwrap_used,clippy::map_unwrap_or)]
//#![warn(clippy::unwrap_in_result)]

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use messages_actix::Blog;
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    Blog::new(8998, "127.0.0.1")
        .run(database_url)
        .unwrap()
        .await
}
