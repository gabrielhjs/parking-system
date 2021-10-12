use super::entity::{Car, ParkingLot};

pub trait ParkingLotRepository {
  fn save_parked_car(&self, car: Car) -> bool;
  fn get_parking_lot(&self, code: &str) -> Option<ParkingLot>;
}
