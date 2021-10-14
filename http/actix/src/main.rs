use std::io;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;

mod enter_parking_lot_controller;
mod get_parking_lot_controller;
use enter_parking_lot_controller::enter_parking_lot_controller;
use get_parking_lot_controller::get_parking_lot_controller;

async fn status() -> HttpResponse {
  #[derive(Serialize)]
  struct Status<'a> {
    pub status: &'a str,
  }

  HttpResponse::Ok().json(Status { status: "Ok" })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/status", web::get().to(status))
      .route("/enter", web::post().to(enter_parking_lot_controller))
      .route("/parking/{code}", web::get().to(get_parking_lot_controller))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
