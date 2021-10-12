use super::super::entity::Car;
use super::super::repository::ParkingLotRepository;
use chrono::Utc;

pub struct EnterParkingLot<R>
where
  R: ParkingLotRepository,
{
  repository: R,
}

impl<R> EnterParkingLot<R>
where
  R: ParkingLotRepository,
{
  pub fn new(repository: R) -> Self {
    EnterParkingLot { repository }
  }
  pub fn execute(&self, code: &str, plate: &str) -> bool {
    match self.repository.get_parking_lot(code) {
      Some(parking_lot) => {
        if parking_lot.is_open() {
          let car = Car {
            code,
            plate,
            enter_date: Utc::now(),
            leave_date: None,
            parked: true,
          };
          self.repository.save_parked_car(car)
        } else {
          false
        }
      }
      None => false,
    }
  }
}
