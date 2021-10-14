use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use parking_system::core::{
  entity::{Car, ParkingLot},
  repository::ParkingLotRepository,
  use_case::enter_parking_lot::EnterParkingLot,
};

struct MockParkingLotRepository;
impl ParkingLotRepository for MockParkingLotRepository {
  fn save_parked_car(&self, _car: Car) -> bool {
    false
  }
  fn get_parking_lot(&self, _code: &str) -> Option<ParkingLot> {
    Some(ParkingLot {
      code: "shopping",
      name: "shopping",
      open_hour: 0,
      close_hour: 23,
    })
  }
}

#[derive(Deserialize)]
pub struct Request {
  pub code: String,
  pub plate: String,
}

#[derive(Serialize)]
struct BadResponse<'a> {
  pub error: &'a str,
}

#[derive(Serialize)]
struct SuccessResponse<'a> {
  pub data: &'a str,
}

pub async fn enter_parking_lot_controller(
  request: web::Json<Request>,
) -> HttpResponse {
  let repository = MockParkingLotRepository {};

  match EnterParkingLot::new(repository)
    .execute(&*request.code, &*request.plate)
  {
    false => HttpResponse::InternalServerError().json(BadResponse {
      error: "Error on save",
    }),
    true => HttpResponse::Ok().json(SuccessResponse { data: "Saved" }),
  }
}
