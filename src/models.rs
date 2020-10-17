use std::time::SystemTime;

use chrono::Utc;
//use crate::schema::posts,
//use crate::schema::comments;
use diesel::prelude::*;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::errors::AppError;
use crate::schema::users;

type Result<T> = std::result::Result<T, AppError>;
type DBConnection = PgConnection;

#[derive(Queryable, Identifiable, Deserialize, Serialize, AsChangeset, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phonenumber: String,

    #[serde(skip_serializing)]
    pub email_verified: bool,

    #[serde(skip_serializing)]
    pub created_at: chrono::DateTime<Utc>,

    #[serde(skip_serializing)]
    pub senha: String,
}

#[derive(Deserialize, Debug, PartialEq)]
// #[table_name = "users"]
pub struct UserOptionals {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,

    #[serde(skip_serializing)]
    pub email_verified: Option<bool>,

    #[serde(skip_serializing)]
    pub created_at: Option<chrono::DateTime<Utc>>,

    // #[serde(skip_serializing)]
    pub senha: Option<String>,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "users"]
pub struct UserToCreate {
    pub name: String,
    pub email: String,
    pub phonenumber: String,
}


/*#[derive(Queryable, Associations, Identifiable, Serialize, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Debug)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
}*/

pub fn create_user(conn: &DBConnection, user: UserToCreate) -> Result<User> {
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .map_err(Into::into)
}

pub fn update_user(conn: &DBConnection, user: UserOptionals) -> Result<User> {
    if user.id.is_none() {
        return Err(AppError::RecordNotFound);
    }

    let id = user.id.unwrap();

    let mut old_user = find_user(conn, UserKey::ID(id))?;

    if let Some(name) = user.name { old_user.name = name; }

    if let Some(email) = user.email { old_user.email = email; }

    if let Some(phonenumber) = user.phonenumber { old_user.phonenumber = phonenumber; }

    diesel::update(users::table.filter(users::id.eq(id)))
        .set(&old_user)
        .get_result(conn)
        .map_err(Into::into)
}

pub enum UserKey<'a> {
    Email(&'a str),
    ID(i32),
}

pub fn list_users(conn: &DBConnection) -> Result<Vec<User>> {
    let vec = users::table
        .select(users::all_columns)
        .load(conn)?;
    Ok(vec)
}

pub fn find_user(conn: &DBConnection, key: UserKey) -> Result<User> {
    match key {
        UserKey::Email(email) => users::table
            .filter(users::email.eq(email))
            .select(users::all_columns)
            .first::<User>(conn)
            .map_err(AppError::from),

        UserKey::ID(id) => users::table
            .find(id)
            .select(users::all_columns)
            .first::<User>(conn)
            .map_err(Into::into),
    }
}

/*pub fn create_post(
    conn: &DBConnection,
    user_var: &User,
    title_var: &str,
    body_var: &str,
) -> Result<Post> {
    conn.transaction(|| {
        diesel::insert_into(posts::table)
            .values((
                posts::user_id.eq(user_var.id),
                posts::title.eq(title_var),
                posts::body.eq(body_var),
            ))
            .execute(conn)?;

        posts::table
            .order(posts::id.desc())
            .select(posts::all_columns)
            .first(conn)
            .map_err(Into::into)
}

pub fn publish_post(
    conn: &DBConnection,
    post_id: i32,
) -> Result<Post> {
    conn.transaction(|| {
        diesel::update(posts::table.filter(posts::id.eq(post_id)))
            .set(posts::published.eq(true))
            .execute(conn)?;

        posts::table
            .find(post_id)
            .select(posts::all_columns)
            .first(conn)
            .map_err(Into::into)
    })
}

pub fn all_published_posts(conn: &DBConnection)
-> Result<Vec<((Post, User), Vec<(Comment, User)>)>> {
    let query = posts::table
        .order(posts::id.desc())
        .filter(posts::published.eq(true))
        .inner_join(users::table)
        .select((posts::all_columns, (users::id, users::username)));
    let posts_with_user = query.load::<(Post, User)>(conn)?;
    let (posts, post_users): (Vec<_>, Vec<_>) = posts_with_user.into_iter().unzip();

    let comments = Comment::belonging_to(&posts)
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)?
        .grouped_by(&posts);

        Ok(posts.into_iter().zip(post_users).zip(comments).collect())
}

pub fn user_posts(
    conn: &DBConnection,
    user_id: i32,
) -> Result<Vec<Post>> {
    posts::table
        .filter(posts::user_id.eq(user_id))
        .order(posts::id.desc())
        .select(posts::all_columns)
        .load::<Post>(conn)
        .map_err(Into::into)
}

pub fn create_comment(
    conn: &DBConnection,
    user_id_var: i32,
    post_id_var: i32,
    body_var: &str,
) -> Result<Comment> {
    conn.transaction(|| {
        diesel::insert_into(comments::table)
            .values((
                comments::user_id.eq(user_id_var),
                comments::post_id.eq(post_id_var),
                comments::body.eq(body_var),
            ))
            .execute(conn)?;

        comments::table
            .order(comments::id.desc())
            .select(comments::all_columns)
            .first(conn)
            .map_err(Into::into)
    })
}

pub fn post_comments(
    conn: &DBConnection,
    post_id_var: i32,
) -> Result<Vec<(Comment, User)>> {
    comments::table
        .filter(comments::post_id.eq(post_id_var))
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)
        .map_err(Into::into)
}

#[derive(Queryable, Serialize, Debug)]
pub struct PostWithComment {
    pub id: i32,
    pub title: String,
    pub published: bool,
}

pub fn user_comments(
    conn: &DBConnection,
    user_id_var: i32,
) -> Result<Vec<(Comment, PostWithComment)>> {
    comments::table
        .filter(comments::user_id.eq(user_id_var))
        .inner_join(posts::table)
        .select((
            comments::all_columns,
            (posts::id, posts::title, posts::published),
        ))
        .load::<(Comment, PostWithComment)>(conn)
        .map_err(Into::into)
}*/
