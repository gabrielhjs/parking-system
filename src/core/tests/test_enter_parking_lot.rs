use super::super::entity::{Car, ParkingLot};
use super::super::repository::ParkingLotRepository;
use super::super::use_case::enter_parking_lot::EnterParkingLot;

struct MockParkingLotRepository;
impl ParkingLotRepository for MockParkingLotRepository {
  fn save_parked_car(&self, _car: Car) -> bool {
    true
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

#[test]
pub fn test_enter_parking_lot() {
  let repository = MockParkingLotRepository {};
  let result = EnterParkingLot::new(repository).execute("shopping", "asd-1234");
  assert_eq!(result, true);
}
