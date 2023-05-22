use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
use crate::models::{NewUser, User};
use crate::schema::users;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserResponse {
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUsersResponse {
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserByIdRequest {
    pub user_id: i32,
}

async fn get_users(db_pool: web::Data<DbPool>) -> impl Responder {
    let db_conn = db_pool.get().expect("Failed to get database connection.");
    let users =users::table.load::<User>(&db_conn).expect("Failed to fetch users");

    HttpResponse::Ok().json(GetUsersResponse { users })
}

async fn create_user(
    db_pool: web::Data<DbPool>,
    req: web::Json<CreateUserRequest>,
) -> impl Responder {
    let db_conn = db_pool.get().expect("Failed to get database connection.");

    let new_user = NewUser {
        name: req.name.clone(),
        email: req.email.clone(),
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&db_conn)
        .expect("Failed to create user");

    HttpResponse::Created().json(GetUserResponse { user })
}

async fn get_user_by_id(
    db_pool: web::Data<DbPool>,
    req: web::Json<GetUserByIdRequest>,
) -> impl Responder {
    let db_conn = db_pool.get().expect("Failed to get database connection.");

    let user = users::table
        .find(req.user_id)
        .first::<User>(&db_conn)
        .expect("Failed to fetch user");

    HttpResponse::Ok().json(GetUserResponse { user })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(get_users)).route(web::post().to(create_user)));
    cfg.service(web::resource("/users/{user_id}").route(web::get().to(get_user_by_id)));
}