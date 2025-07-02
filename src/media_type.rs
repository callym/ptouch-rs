use nom::{IResult, number::complete::u8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
  None,
  Laminated,
  NonLaminated,
  Fabric,
  HeatShrink,
  Fle,
  FlexibleId,
  Satin,
  Incompatible,
  Unknown(u8),
}

impl From<u8> for MediaType {
  fn from(value: u8) -> Self {
    match value {
      0x00 => Self::None,
      0x01 => Self::Laminated,
      0x03 => Self::NonLaminated,
      0x04 => Self::Fabric,
      0x11 => Self::HeatShrink,
      0x13 => Self::Fle,
      0x14 => Self::FlexibleId,
      0x15 => Self::Satin,
      0xFF => Self::Incompatible,
      v => Self::Unknown(v),
    }
  }
}

impl MediaType {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, media_type) = u8(input)?;

    Ok((input, media_type.into()))
  }
}
