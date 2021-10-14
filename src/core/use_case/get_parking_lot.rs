use super::super::entity::ParkingLot;
use super::super::repository::ParkingLotRepository;

pub struct GetParkingLot<R>
where
  R: ParkingLotRepository,
{
  repository: R,
}
impl<R> GetParkingLot<R>
where
  R: ParkingLotRepository,
{
  pub fn new(repository: R) -> Self {
    GetParkingLot { repository }
  }
  pub fn execute(&self, code: &str) -> Option<ParkingLot> {
    self.repository.get_parking_lot(code)
  }
}
