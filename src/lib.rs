#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

use actix_web::{App, middleware, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod errors;
mod models;
mod routes;
mod schema;

pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16) -> Self {
        Blog { port }
    }

    pub fn run(&self, database_url: String) -> std::io::Result<()> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
    
            println!("Starting http server: 127.0.0.1:{}", self.port);
            HttpServer::new(move || {
                App::new()
                    .data(pool.clone())
                    .wrap(middleware::Logger::default())
                    .configure(routes::users::configure)
            })
            .bind(("127.0.0.1", self.port))?
            .run()
    }
}

/*
use actix_web::{
    error::{Error, InternalError, JsonPayloadError},
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const LOG_FORMAT: &'static str = r#""%r" %s %b %{User-Agent}i %D"#;
static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

pub struct MessageApp {
    port: u16,    
}

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms_clone;
    {
        ms_clone = state.messages.lock().unwrap().clone();
    }
    
    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms_clone,
    }))
}

fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    {
        let mut ms = state.messages.lock().unwrap();
        ms.push(msg.message.clone());
    }
    
    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: msg.message.clone(),
    }))
}

#[post("/clear")]
fn clear (state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    {
        let mut ms = state.messages.lock().unwrap();
        ms.clear();
    }
    
    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: vec![],
    }))
}

#[derive(Serialize)]
struct LookupResponse {
    server_id: usize,
    request_count: usize,
    result: Option<String>,
}

#[get("/lookup/{index}")]
fn lookup(state: web::Data<AppState>, idx: web::Path<usize>) -> Result<web::Json<LookupResponse>> {
    let request_count = state.request_count.get()+1;
    state.request_count.set(request_count);
    let result;
    {
        let ms = state.messages.lock().unwrap();
        result = ms.get(idx.into_inner()).cloned();
    }
    Ok (web::Json(LookupResponse {
        server_id: state.server_id,
        request_count,
        result,
    }))
}

fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    let extns = req.extensions();
    println!("{:?}", req);
    println!("{:?}", err);
    println!("{:?}", extns);
    let state = extns.get::<web::Data<AppState>>();
    match state  {
        Some(st) => {
            let request_count = st.request_count.get()+1;
            st.request_count.set(request_count);
            let post_error = PostError {
                server_id: st.server_id,
                request_count,
                error:format!("{}", err),
            };
            return InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into();
        }
        None => {
            panic!("Erro estranho");
            
        }
    }
    
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }
    
    pub fn run(&self) -> std::io::Result<()> {
        let messages = Arc::new(Mutex::new(vec![]));
        
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::new(LOG_FORMAT))
                .service(index)
                .service(
                    web::resource("/send")
                        .data(web::JsonConfig::default()
                            .limit(1024)
                            .error_handler(post_error),
                        )
                        .route(web::post().to(post)),
                )
                .service(clear)
                .service(lookup)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(3)
        .run()
    }
}
*/