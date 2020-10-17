extern crate diesel;

use actix_web::{App, get, post, put, delete, HttpServer};
use actix_web::{HttpResponse, Responder, web};

use crate::{models, Pool};
use crate::errors::AppError;
use crate::models::{User, UserToCreate, UserOptionals};
use crate::routes::convert;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_user)
        .service(get_user)
        .service(list_users)
        .service(update_user);
}

#[post("/users")]
async fn create_user(item: web::Json<UserToCreate>, pool: web::Data<Pool>)
                     -> impl Responder {
    let conn = pool.get().unwrap();
    let user = item.into_inner();

    let user = web::block(move || models::create_user(&conn, user))
        .await;

    convert(user)
}

#[put("/users")]
async fn update_user(item: web::Json<UserOptionals>, pool: web::Data<Pool>)
                     -> impl Responder {
    let conn = pool.get().unwrap();
    let user = item.into_inner();

    let user = web::block(move || models::update_user(&conn, user))
        .await;

    convert(user)
}

/*#[get("/users/find/{name}")]
async fn find_user(name: web::Path<String>, pool: web::Data<Pool>)
    -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let name = name.into_inner();
        let key = models::UserKey::Username(name.as_str());
        models::find_user(conn, key)
    })
    .then(convert)
}*/


#[get("/users")]
async fn list_users(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().unwrap();

    let response = web::block(move || models::list_users(&conn))
        .await;

    convert(response)
}

#[get("/users/{id}")]
async fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let user_id = user_id.into_inner();
    let conn = pool.get().unwrap();
    let key = models::UserKey::ID(user_id);

    let user = web::block(move || models::find_user(&conn, key))
        .await;

    convert(user)
}

