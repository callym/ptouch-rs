use nom::{IResult, number::complete::u8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatusType {
  Ok,
  TapeDoorOpen,
  Unknown(u8),
}

impl From<u8> for StatusType {
  fn from(value: u8) -> Self {
    match value {
      0x00 => StatusType::Ok,
      0x02 => StatusType::TapeDoorOpen,
      v => StatusType::Unknown(v),
    }
  }
}

impl StatusType {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, color) = u8(input)?;

    Ok((input, color.into()))
  }
}
