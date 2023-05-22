use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod api;
mod config;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env();
    let db_pool = db::create_db_pool(&config);

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(api::users::get_users)
            .service(api::users::create_user)
            .service(api::users::get_user_by_id)
            .service(api::posts::get_posts)
            .service(api::posts::create_post)
            .service(api::posts::get_post_by_id)
            .service(api::posts::delete_post_by_id)
    })
    .bind(format!("{}:{}", config.server_host, config.server_port))?
    .run()
    .await
}