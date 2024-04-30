use super::*;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, DeserializeFromStr, SerializeDisplay)]
pub enum Rarity {
  Common,
  Uncommon,
  Rare,
  Epic,
  Legendary,
  Mythic,
}

impl From<Rarity> for u8 {
  fn from(rarity: Rarity) -> Self {
    rarity as u8
  }
}

impl TryFrom<u8> for Rarity {
  type Error = u8;

  fn try_from(rarity: u8) -> Result<Self, u8> {
    match rarity {
      0 => Ok(Self::Common),
      1 => Ok(Self::Uncommon),
      2 => Ok(Self::Rare),
      3 => Ok(Self::Epic),
      4 => Ok(Self::Legendary),
      5 => Ok(Self::Mythic),
      n => Err(n),
    }
  }
}

impl Display for Rarity {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Common => "common",
        Self::Uncommon => "uncommon",
        Self::Rare => "rare",
        Self::Epic => "epic",
        Self::Legendary => "legendary",
        Self::Mythic => "mythic",
      }
    )
  }
}

impl From<Sat> for Rarity {
  fn from(sat: Sat) -> Self {
    let Degree {
      hour,
      minute,
      second,
      third,
    } = sat.degree();

    if hour == 0 && minute == 0 && second == 0 && third == 0 {
      Self::Mythic
    } else if minute == 0 && second == 0 && third == 0 {
      Self::Legendary
    } else if minute == 0 && third == 0 {
      Self::Epic
    } else if second == 0 && third == 0 {
      Self::Rare
    } else if third == 0 {
      Self::Uncommon
    } else {
      Self::Common
    }
  }
}

impl FromStr for Rarity {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "common" => Ok(Self::Common),
      "uncommon" => Ok(Self::Uncommon),
      "rare" => Ok(Self::Rare),
      "epic" => Ok(Self::Epic),
      "legendary" => Ok(Self::Legendary),
      "mythic" => Ok(Self::Mythic),
      _ => Err(format!("invalid rarity `{s}`")),
    }
  }
}
 