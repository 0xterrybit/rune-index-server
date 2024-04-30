use super::*;

#[derive(PartialEq, Debug)]
pub struct DecimalSat {
  pub height: Height,
  pub offset: u64,
}

impl From<Sat> for DecimalSat {
  fn from(sat: Sat) -> Self {
    Self {
      height: sat.height(),
      offset: sat.third(),
    }
  }
}

impl Display for DecimalSat {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}.{}", self.height, self.offset)
  }
}