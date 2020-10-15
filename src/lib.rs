#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{App, HttpServer, middleware};
use actix_web::dev::Server;
use actix_web::http::ContentEncoding;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod errors;
mod models;
mod routes;
mod schema;

pub struct Blog {
    port: u16,
    host: &'static str,
}

impl Blog {
    pub fn new(port: u16, host: &'static str) -> Self {
        Blog {
            port,
            host,
        }
    }

    pub fn run(&self, database_url: String) -> std::io::Result<Server> {
/*        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let manager = Pool::new(manager);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");*/

        let (host, port) = (self.host, self.port);
        // let (host, port) = ("0.0.0.0", 8998);

        println!("Starting http server: {}:{}", host, { port });
        let server = HttpServer::new(move ||
                            App::new()
                                // .data(pool.clone())
                                //.wrap(middleware::Compress::new(ContentEncoding::Br))
                                //.wrap(middleware::Logger::default())
                                .configure(routes::users::configure)
                        //.configure(routes::posts::configure)
                        //.configure(routes::comments::configure)
        )
            .bind((host, port))?
            .run();
        
        Ok(server)

    }
}
