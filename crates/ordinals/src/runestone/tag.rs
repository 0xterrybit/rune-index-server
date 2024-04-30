use super::*;

#[derive(Copy, Clone, Debug)]
pub(super) enum Tag {
  Body = 0,
  Flags = 2,
  Rune = 4,
  Premine = 6,
  Cap = 8,
  Amount = 10,
  HeightStart = 12,
  HeightEnd = 14,
  OffsetStart = 16,
  OffsetEnd = 18,
  Mint = 20,
  Pointer = 22,
  #[allow(unused)]
  Cenotaph = 126,

  Divisibility = 1,
  Spacers = 3,
  Symbol = 5,
  #[allow(unused)]
  Nop = 127,
}

impl Tag {
  pub(super) fn take<const N: usize, T>(
    self,
    fields: &mut HashMap<u128, VecDeque<u128>>,
    with: impl Fn([u128; N]) -> Option<T>,
  ) -> Option<T> {
    let field = fields.get_mut(&self.into())?;

    let mut values: [u128; N] = [0; N];

    for (i, v) in values.iter_mut().enumerate() {
      *v = *field.get(i)?;
    }

    let value = with(values)?;

    field.drain(0..N);

    if field.is_empty() {
      fields.remove(&self.into()).unwrap();
    }

    Some(value)
  }

  pub(super) fn encode<const N: usize>(self, values: [u128; N], payload: &mut Vec<u8>) {
    for value in values {
      varint::encode_to_vec(self.into(), payload);
      varint::encode_to_vec(value, payload);
    }
  }

  pub(super) fn encode_option<T: Into<u128>>(self, value: Option<T>, payload: &mut Vec<u8>) {
    if let Some(value) = value {
      self.encode([value.into()], payload)
    }
  }
}

impl From<Tag> for u128 {
  fn from(tag: Tag) -> Self {
    tag as u128
  }
}

impl PartialEq<u128> for Tag {
  fn eq(&self, other: &u128) -> bool {
    u128::from(*self) == *other
  }
}

