use actix_web::{web, HttpResponse};
use serde::Serialize;

use parking_system::core::{
  entity::{Car, ParkingLot},
  repository::ParkingLotRepository,
  use_case::get_parking_lot::GetParkingLot,
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

pub async fn get_parking_lot_controller(
  path: web::Path<(String,)>,
) -> HttpResponse {
  let repository = MockParkingLotRepository {};

  match GetParkingLot::new(repository).execute(&*path.into_inner().0) {
    Some(parking_lot) => {
      #[derive(Serialize)]
      struct Response<'a> {
        code: &'a str,
        name: &'a str,
        open_hour: u8,
        close_hour: u8,
      }
      HttpResponse::Ok().json(Response {
        code: parking_lot.code,
        name: parking_lot.name,
        open_hour: parking_lot.open_hour,
        close_hour: parking_lot.open_hour,
      })
    }
    None => HttpResponse::NotFound().finish(),
  }
}
