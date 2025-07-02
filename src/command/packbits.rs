pub struct PackBits;

impl PackBits {
  pub const fn message() -> &'static [u8] {
    b"M\x02"
  }
}
