use chrono::{DateTime, Local, Timelike, Utc};

pub struct ParkingLot<'a> {
  pub code: &'a str,
  pub name: &'a str,
  pub open_hour: u8,
  pub close_hour: u8,
}

impl<'a> ParkingLot<'a> {
  pub fn is_open(&self) -> bool {
    let local_hour = Local::now().hour();
    local_hour.ge(&self.open_hour.into())
      && local_hour.le(&self.close_hour.into())
  }
}

pub struct Car<'a> {
  pub code: &'a str,
  pub plate: &'a str,
  pub enter_date: DateTime<Utc>,
  pub leave_date: Option<DateTime<Utc>>,
  pub parked: bool,
}

impl<'a> Car<'a> {
  pub fn new(code: &'a str, plate: &'a str) -> Self {
    Car {
      code,
      plate,
      enter_date: Utc::now(),
      leave_date: None,
      parked: true,
    }
  }
}
