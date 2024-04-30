pub(crate) trait IntoUsize {
  fn into_usize(self) -> usize;
}

impl IntoUsize for u32 {
  fn into_usize(self) -> usize {
    self.try_into().unwrap()
  }
}