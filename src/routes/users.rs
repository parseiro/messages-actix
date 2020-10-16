extern crate diesel;

use actix_web::{App, get, HttpServer};
use actix_web::{HttpResponse, Responder, web};

use crate::{models, Pool};
use crate::errors::AppError;
use crate::models::User;
use crate::routes::convert;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
// .service(web::resource("/users").route(web::post().to_async(create_user)))
//        .service(web::resource("/users/find/{name}").route(web::get().to_async(find_user)))
//         .service(web::resource("/users/{id}").route(web::get().to_async(get_user)));
        .service(get_user)
        .service(list_users);
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

async fn create_user(item: web::Json<UserInput>, pool: web::Data<Pool>)
-> impl Responder {
    let conn = pool.get().unwrap();
    let username = item.into_inner().username;

    let user = web::block(move || models::create_user(&conn, username.as_str())).await
        .map_err(|e| {
            eprintln!("{:?}", e);
            e
        });

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
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            e
        });

    convert(response)
}

#[get("/users/{id}")]
async fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let user_id = user_id.into_inner();
    let conn = pool.get().unwrap();
    let key = models::UserKey::ID(user_id);


    //models::find_user(&pool.get().unwrap(), models::UserKey::ID(user_id.into_inner()))
    //format!("Hello {:?}", user_id);

    let user = web::block(move || models::find_user(&conn, key))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            e
            //HttpResponse::InternalServerError().finish()
        });

    convert(user)
}

