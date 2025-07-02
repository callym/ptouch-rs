use nom::{
  IResult,
  error::{ErrorKind, FromExternalError},
  number::complete::u8,
};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TapeInfo {
  pub px: u32,
  pub margins: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum TapeSize {
  ThreePointFive,
  Six,
  Nine,
  Twelve,
  Eighteen,
  TwentyFour,
  ThirtySix,
}

impl TapeSize {
  pub const fn info(&self) -> TapeInfo {
    match self {
      TapeSize::ThreePointFive => TapeInfo {
        px: 24,
        margins: 0.5,
      },
      TapeSize::Six => TapeInfo {
        px: 32,
        margins: 1.0,
      },
      TapeSize::Nine => TapeInfo {
        px: 52,
        margins: 1.0,
      },
      TapeSize::Twelve => TapeInfo {
        px: 76,
        margins: 2.0,
      },
      TapeSize::Eighteen => TapeInfo {
        px: 120,
        margins: 3.0,
      },
      TapeSize::TwentyFour => TapeInfo {
        px: 128,
        margins: 3.0,
      },
      TapeSize::ThirtySix => TapeInfo {
        px: 192,
        margins: 4.5,
      },
    }
  }
}

impl From<TapeSize> for u8 {
  fn from(val: TapeSize) -> Self {
    match val {
      TapeSize::ThreePointFive => 4,
      TapeSize::Six => 6,
      TapeSize::Nine => 9,
      TapeSize::Twelve => 12,
      TapeSize::Eighteen => 18,
      TapeSize::TwentyFour => 24,
      TapeSize::ThirtySix => 36,
    }
  }
}

impl TryFrom<u8> for TapeSize {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      4 => Self::ThreePointFive,
      6 => Self::Six,
      9 => Self::Nine,
      12 => Self::Twelve,
      18 => Self::Eighteen,
      24 => Self::TwentyFour,
      36 => Self::ThirtySix,
      other => Err(Error::InvalidTapeSize(other))?,
    })
  }
}

impl TapeSize {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, color) = u8(input)?;
    let color = match Self::try_from(color) {
      Ok(color) => color,
      Err(e) => {
        let err = nom::error::Error::from_external_error(input, ErrorKind::Fail, e);
        Err(nom::Err::Error(err))?
      },
    };

    Ok((input, color))
  }
}
