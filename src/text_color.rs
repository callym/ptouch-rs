use nom::{IResult, number::complete::u8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextColor {
  None,
  Black,
  Blue,
  BlueF,
  Cleaning,
  Gold,
  Incompatible,
  Red,
  Stencil,
  White,
  Other,
  Unknown(u8),
}

impl From<u8> for TextColor {
  fn from(value: u8) -> Self {
    match value {
      0x00 => TextColor::None,
      0x01 => TextColor::White,
      0x02 => TextColor::Other,
      0x04 => TextColor::Red,
      0x05 => TextColor::Blue,
      0x08 => TextColor::Black,
      0x0a => TextColor::Gold,
      0x62 => TextColor::BlueF,
      0xf0 => TextColor::Cleaning,
      0xf1 => TextColor::Stencil,
      0xff => TextColor::Incompatible,
      v => TextColor::Unknown(v),
    }
  }
}

impl TextColor {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, color) = u8(input)?;

    Ok((input, color.into()))
  }
}
