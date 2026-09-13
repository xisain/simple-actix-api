mod errors;
mod models;
mod routes;
mod state;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use state::AppState;

#[get("/")]
async fn greeting() -> impl Responder {
    HttpResponse::Ok().json("Selamat datang di API Koleksi Tanaman!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::new());

    println!("Server berjalan di http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(greeting)
            .service(routes::tanaman::semua_tanaman)
            .service(routes::tanaman::satu_tanaman)
            .service(routes::tanaman::tambah_tanaman)
            .service(routes::tanaman::update_tanaman)
            .service(routes::tanaman::hapus_tanaman)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
