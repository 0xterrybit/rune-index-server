use super::*;

#[derive(PartialEq, Debug)]
pub struct Degree {
  pub hour: u32,
  pub minute: u32,
  pub second: u32,
  pub third: u64,
}

impl Display for Degree {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}°{}′{}″{}‴",
      self.hour, self.minute, self.second, self.third
    )
  }
}

impl From<Sat> for Degree {
  fn from(sat: Sat) -> Self {
    let height = sat.height().n();
    Degree {
      hour: height / (CYCLE_EPOCHS * SUBSIDY_HALVING_INTERVAL),
      minute: height % SUBSIDY_HALVING_INTERVAL,
      second: height % DIFFCHANGE_INTERVAL,
      third: sat.third(),
    }
  }
}

