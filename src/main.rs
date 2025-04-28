use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self,ConnectionManager};
use dotenvy::dotenv;
use std::env;


mod handlers;
mod models;
mod schema;
mod db;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Optional: uji koneksi ke database saat startup
    let _conn = db::establish_connection();

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/data/handleMsg.do")
                    .route(web::post().to(handlers::handle_msg::handle_msg)),
            )
            .service(
                web::resource("/data/{id}")
                    .route(web::post().to(handlers::data_handler::handle_data)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
