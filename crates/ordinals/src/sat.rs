use {super::*, std::num::ParseFloatError};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Display, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Sat(pub u64);

impl Sat {
  pub const LAST: Self = Self(Self::SUPPLY - 1);
  pub const SUPPLY: u64 = 2099999997690000;

  pub fn n(self) -> u64 {
    self.0
  }

  pub fn degree(self) -> Degree {
    self.into()
  }

  pub fn height(self) -> Height {
    self.epoch().starting_height()
      + u32::try_from(self.epoch_position() / self.epoch().subsidy()).unwrap()
  }

  pub fn cycle(self) -> u32 {
    Epoch::from(self).0 / CYCLE_EPOCHS
  }

  pub fn nineball(self) -> bool {
    self.n() >= 50 * COIN_VALUE * 9 && self.n() < 50 * COIN_VALUE * 10
  }

  pub fn percentile(self) -> String {
    format!("{}%", (self.0 as f64 / Self::LAST.0 as f64) * 100.0)
  }

  pub fn epoch(self) -> Epoch {
    self.into()
  }

  pub fn period(self) -> u32 {
    self.height().n() / DIFFCHANGE_INTERVAL
  }

  pub fn third(self) -> u64 {
    self.epoch_position() % self.epoch().subsidy()
  }

  pub fn epoch_position(self) -> u64 {
    self.0 - self.epoch().starting_sat().0
  }

  pub fn decimal(self) -> DecimalSat {
    self.into()
  }

  pub fn rarity(self) -> Rarity {
    self.into()
  }

  /// `Sat::rarity` is expensive and is called frequently when indexing.
  /// Sat::is_common only checks if self is `Rarity::Common` but is
  /// much faster.
  pub fn common(self) -> bool {
    let epoch = self.epoch();
    (self.0 - epoch.starting_sat().0) % epoch.subsidy() != 0
  }

  pub fn coin(self) -> bool {
    self.n() % COIN_VALUE == 0
  }

  pub fn name(self) -> String {
    let mut x = Self::SUPPLY - self.0;
    let mut name = String::new();
    while x > 0 {
      name.push(
        "abcdefghijklmnopqrstuvwxyz"
          .chars()
          .nth(((x - 1) % 26) as usize)
          .unwrap(),
      );
      x = (x - 1) / 26;
    }
    name.chars().rev().collect()
  }

  pub fn charms(self) -> u16 {
    let mut charms = 0;

    if self.nineball() {
      Charm::Nineball.set(&mut charms);
    }

    if self.coin() {
      Charm::Coin.set(&mut charms);
    }

    match self.rarity() {
      Rarity::Common => {}
      Rarity::Epic => Charm::Epic.set(&mut charms),
      Rarity::Legendary => Charm::Legendary.set(&mut charms),
      Rarity::Mythic => Charm::Mythic.set(&mut charms),
      Rarity::Rare => Charm::Rare.set(&mut charms),
      Rarity::Uncommon => Charm::Uncommon.set(&mut charms),
    }

    charms
  }

  fn from_name(s: &str) -> Result<Self, Error> {
    let mut x = 0;
    for c in s.chars() {
      match c {
        'a'..='z' => {
          x = x * 26 + c as u64 - 'a' as u64 + 1;
          if x > Self::SUPPLY {
            return Err(ErrorKind::NameRange.error(s));
          }
        }
        _ => return Err(ErrorKind::NameCharacter.error(s)),
      }
    }
    Ok(Sat(Self::SUPPLY - x))
  }

  fn from_degree(degree: &str) -> Result<Self, Error> {
    let (cycle_number, rest) = degree
      .split_once('°')
      .ok_or_else(|| ErrorKind::MissingDegree.error(degree))?;

    let cycle_number = cycle_number
      .parse::<u32>()
      .map_err(|source| ErrorKind::ParseInt { source }.error(degree))?;

    let (epoch_offset, rest) = rest
      .split_once('′')
      .ok_or_else(|| ErrorKind::MissingMinute.error(degree))?;

    let epoch_offset = epoch_offset
      .parse::<u32>()
      .map_err(|source| ErrorKind::ParseInt { source }.error(degree))?;

    if epoch_offset >= SUBSIDY_HALVING_INTERVAL {
      return Err(ErrorKind::EpochOffset.error(degree));
    }

    let (period_offset, rest) = rest
      .split_once('″')
      .ok_or_else(|| ErrorKind::MissingSecond.error(degree))?;

    let period_offset = period_offset
      .parse::<u32>()
      .map_err(|source| ErrorKind::ParseInt { source }.error(degree))?;

    if period_offset >= DIFFCHANGE_INTERVAL {
      return Err(ErrorKind::PeriodOffset.error(degree));
    }

    let cycle_start_epoch = cycle_number * CYCLE_EPOCHS;

    const HALVING_INCREMENT: u32 = SUBSIDY_HALVING_INTERVAL % DIFFCHANGE_INTERVAL;

    // For valid degrees the relationship between epoch_offset and period_offset
    // will increment by 336 every halving.
    let relationship = period_offset + SUBSIDY_HALVING_INTERVAL * CYCLE_EPOCHS - epoch_offset;

    if relationship % HALVING_INCREMENT != 0 {
      return Err(ErrorKind::EpochPeriodMismatch.error(degree));
    }

    let epochs_since_cycle_start = relationship % DIFFCHANGE_INTERVAL / HALVING_INCREMENT;

    let epoch = cycle_start_epoch + epochs_since_cycle_start;

    let height = Height(epoch * SUBSIDY_HALVING_INTERVAL + epoch_offset);

    let (block_offset, rest) = match rest.split_once('‴') {
      Some((block_offset, rest)) => (
        block_offset
          .parse::<u64>()
          .map_err(|source| ErrorKind::ParseInt { source }.error(degree))?,
        rest,
      ),
      None => (0, rest),
    };

    if !rest.is_empty() {
      return Err(ErrorKind::TrailingCharacters.error(degree));
    }

    if block_offset >= height.subsidy() {
      return Err(ErrorKind::BlockOffset.error(degree));
    }

    Ok(height.starting_sat() + block_offset)
  }

  fn from_decimal(decimal: &str) -> Result<Self, Error> {
    let (height, offset) = decimal
      .split_once('.')
      .ok_or_else(|| ErrorKind::MissingPeriod.error(decimal))?;

    let height = Height(
      height
        .parse()
        .map_err(|source| ErrorKind::ParseInt { source }.error(decimal))?,
    );

    let offset = offset
      .parse::<u64>()
      .map_err(|source| ErrorKind::ParseInt { source }.error(decimal))?;

    if offset >= height.subsidy() {
      return Err(ErrorKind::BlockOffset.error(decimal));
    }

    Ok(height.starting_sat() + offset)
  }

  fn from_percentile(percentile: &str) -> Result<Self, Error> {
    if !percentile.ends_with('%') {
      return Err(ErrorKind::Percentile.error(percentile));
    }

    let percentile_string = percentile;

    let percentile = percentile[..percentile.len() - 1]
      .parse::<f64>()
      .map_err(|source| ErrorKind::ParseFloat { source }.error(percentile))?;

    if percentile < 0.0 {
      return Err(ErrorKind::Percentile.error(percentile_string));
    }

    let last = Sat::LAST.n() as f64;

    let n = (percentile / 100.0 * last).round();

    if n > last {
      return Err(ErrorKind::Percentile.error(percentile_string));
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    Ok(Sat(n as u64))
  }
}

#[derive(Debug, Error)]
pub struct Error {
  input: String,
  kind: ErrorKind,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "failed to parse sat `{}`: {}", self.input, self.kind)
  }
}

#[derive(Debug, Error)]
pub enum ErrorKind {
  IntegerRange,
  NameRange,
  NameCharacter,
  Percentile,
  BlockOffset,
  MissingPeriod,
  TrailingCharacters,
  MissingDegree,
  MissingMinute,
  MissingSecond,
  PeriodOffset,
  EpochOffset,
  EpochPeriodMismatch,
  ParseInt { source: ParseIntError },
  ParseFloat { source: ParseFloatError },
}

impl ErrorKind {
  fn error(self, input: &str) -> Error {
    Error {
      input: input.to_string(),
      kind: self,
    }
  }
}

impl Display for ErrorKind {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::IntegerRange => write!(f, "invalid integer range"),
      Self::NameRange => write!(f, "invalid name range"),
      Self::NameCharacter => write!(f, "invalid character in name"),
      Self::Percentile => write!(f, "invalid percentile"),
      Self::BlockOffset => write!(f, "invalid block offset"),
      Self::MissingPeriod => write!(f, "missing period"),
      Self::TrailingCharacters => write!(f, "trailing character"),
      Self::MissingDegree => write!(f, "missing degree symbol"),
      Self::MissingMinute => write!(f, "missing minute symbol"),
      Self::MissingSecond => write!(f, "missing second symbol"),
      Self::PeriodOffset => write!(f, "invalid period offset"),
      Self::EpochOffset => write!(f, "invalid epoch offset"),
      Self::EpochPeriodMismatch => write!(
        f,
        "relationship between epoch offset and period offset must be multiple of 336"
      ),
      Self::ParseInt { source } => write!(f, "invalid integer: {source}"),
      Self::ParseFloat { source } => write!(f, "invalid float: {source}"),
    }
  }
}

impl PartialEq<u64> for Sat {
  fn eq(&self, other: &u64) -> bool {
    self.0 == *other
  }
}

impl PartialOrd<u64> for Sat {
  fn partial_cmp(&self, other: &u64) -> Option<cmp::Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add<u64> for Sat {
  type Output = Self;

  fn add(self, other: u64) -> Sat {
    Sat(self.0 + other)
  }
}

impl AddAssign<u64> for Sat {
  fn add_assign(&mut self, other: u64) {
    *self = Sat(self.0 + other);
  }
}

impl FromStr for Sat {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.chars().any(|c| c.is_ascii_lowercase()) {
      Self::from_name(s)
    } else if s.contains('°') {
      Self::from_degree(s)
    } else if s.contains('%') {
      Self::from_percentile(s)
    } else if s.contains('.') {
      Self::from_decimal(s)
    } else {
      let sat = Self(
        s.parse()
          .map_err(|source| ErrorKind::ParseInt { source }.error(s))?,
      );
      if sat > Self::LAST {
        Err(ErrorKind::IntegerRange.error(s))
      } else {
        Ok(sat)
      }
    }
  }
}
 