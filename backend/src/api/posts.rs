use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
use crate::models::{NewPost, Post};
use crate::schema::posts;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetPostResponse {
    pub post: Post,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPostByIdRequest {
    pub post_id: i32,
}

async fn get_posts(db_pool: web::Data<DbPool>) -> impl Responder {
    let db_conn = db_pool.get().expect("Failed to get database connection.");
    let posts = posts::table.load::<Post>(&db_conn).expect("Failed to fetch posts");

    HttpResponse::Ok().json(GetPostsResponse { posts })
}

async fn create_post(
    db_pool: web::Data<DbPool>,
    req: web::Json<CreatePostRequest>,
) -> impl Responder {
    let db_conn = db_pool.get().expect("Failed to get database connection.");

    let new_post = NewPost {
        title: req.title.clone(),
        body: req.body.clone(),
        user_id: req.user_id,
    };

    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&db_conn)
        .expect("Failed to create post");

    HttpResponse::Created().json(GetPostResponse { post })
}

async fn get_post_by_id(
    db_pool: web::Data<DbPool>,
    req: web::Json<GetPostByIdRequest>,
) -> impl Responder {
    let db_conn = db_pool.get().expect("Failed to get database connection.");

    let post = posts::table
        .find(req.post_id)
        .first::<Post>(&db_conn)
        .expect("Failed to fetch post");

    HttpResponse::Ok().json(GetPostResponse { post })
}

async fn delete_post_by_id(
    db_pool: web::Data<DbPool>,
    req: web::Json<GetPostByIdRequest>,
) -> impl Responder {
    let db_conn = db_pool.get().expect("Failed to get database connection.");

    let num_deleted = diesel::delete(posts