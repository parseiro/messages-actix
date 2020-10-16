use crate::errors::AppError;
use crate::schema::users;
//use crate::schema::posts, 
//use crate::schema::comments;
use diesel::prelude::*;

type Result<T> = std::result::Result<T, AppError>;
type DBConnection = PgConnection;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
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

pub fn create_user(
//    conn: &DBConnection,
    conn: &DBConnection,
    username: &str,
) -> Result<User> {
    conn.transaction(|| {
        diesel::insert_into(users::table)
            .values((users::name.eq(username),))
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .select((users::id, users::name))
            .first(conn)
            .map_err(Into::into)
    })
}

pub enum UserKey<'a> {
    Username(&'a str),
    ID(i32),
}

pub fn list_users(conn: &DBConnection) -> Option<List<User>> {
    let a = users::table
        .select((users::id, users::name));

    if let Some(list) = a {

    }
}

pub fn find_user(
    conn: &DBConnection,
    key: UserKey,
) -> Result<User> {
    match key {
        UserKey::Username(name) => users::table
            .filter(users::name.eq(name))
            .select((users::id, users::name))
            .first::<User>(conn)
            .map_err(AppError::from),
        
        UserKey::ID(id) => users::table
            .find(id)
            .select((users::id, users::name))
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
    })
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
