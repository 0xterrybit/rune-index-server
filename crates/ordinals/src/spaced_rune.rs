use super::*;

#[derive(
  Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Default, DeserializeFromStr, SerializeDisplay,
)]
pub struct SpacedRune {
  pub rune: Rune,
  pub spacers: u32,
}

impl SpacedRune {
  pub fn new(rune: Rune, spacers: u32) -> Self {
    Self { rune, spacers }
  }
}

impl FromStr for SpacedRune {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut rune = String::new();
    let mut spacers = 0u32;

    for c in s.chars() {
      match c {
        'A'..='Z' => rune.push(c),
        '.' | '•' => {
          let flag = 1 << rune.len().checked_sub(1).ok_or(Error::LeadingSpacer)?;
          if spacers & flag != 0 {
            return Err(Error::DoubleSpacer);
          }
          spacers |= flag;
        }
        _ => return Err(Error::Character(c)),
      }
    }

    if 32 - spacers.leading_zeros() >= rune.len().try_into().unwrap() {
      return Err(Error::TrailingSpacer);
    }

    Ok(SpacedRune {
      rune: rune.parse().map_err(Error::Rune)?,
      spacers,
    })
  }
}

impl Display for SpacedRune {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let rune = self.rune.to_string();

    for (i, c) in rune.chars().enumerate() {
      write!(f, "{c}")?;

      if i < rune.len() - 1 && self.spacers & 1 << i != 0 {
        write!(f, "•")?;
      }
    }

    Ok(())
  }
}

#[derive(Debug, PartialEq)]
pub enum Error {
  LeadingSpacer,
  TrailingSpacer,
  DoubleSpacer,
  Character(char),
  Rune(rune::Error),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Character(c) => write!(f, "invalid character `{c}`"),
      Self::DoubleSpacer => write!(f, "double spacer"),
      Self::LeadingSpacer => write!(f, "leading spacer"),
      Self::TrailingSpacer => write!(f, "trailing spacer"),
      Self::Rune(err) => write!(f, "{err}"),
    }
  }
}

impl std::error::Error for Error {}
