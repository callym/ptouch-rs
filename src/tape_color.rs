use nom::{IResult, number::complete::u8};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TapeColor {
  None,
  BerryPink_TZe_MQP35,
  Black,
  Blue_TZe_5_345_5,
  Blue,
  Cleaning,
  Clear,
  ClearMatte,
  GoldSatin,
  Green,
  HeatShrinkTube,
  Incompatible,
  LightGray_TZe_MQL35,
  LimeGreen_TZe_MQG35,
  OrangeFluorescent,
  Pink,
  Red_TZe_435,
  Red,
  SilverMatte,
  SilverSatin,
  Stencil,
  White,
  WhiteFlexId,
  WhiteMatte,
  Yellow,
  YellowFlexId,
  YellowFluorescent,
  Other,
  Unknown(u8),
}

impl From<u8> for TapeColor {
  fn from(value: u8) -> Self {
    match value {
      0x00 => TapeColor::None,
      0x01 => TapeColor::White,
      0x02 => TapeColor::Other,
      0x03 => TapeColor::Clear,
      0x04 => TapeColor::Red,
      0x05 => TapeColor::Blue,
      0x06 => TapeColor::Yellow,
      0x07 => TapeColor::Green,
      0x08 => TapeColor::Black,
      0x09 => TapeColor::Clear,
      0x20 => TapeColor::WhiteMatte,
      0x21 => TapeColor::ClearMatte,
      0x22 => TapeColor::SilverMatte,
      0x23 => TapeColor::GoldSatin,
      0x24 => TapeColor::SilverSatin,
      0x30 => TapeColor::Blue_TZe_5_345_5,
      0x31 => TapeColor::Red_TZe_435,
      0x40 => TapeColor::OrangeFluorescent,
      0x41 => TapeColor::YellowFluorescent,
      0x50 => TapeColor::BerryPink_TZe_MQP35,
      0x51 => TapeColor::LightGray_TZe_MQL35,
      0x52 => TapeColor::LimeGreen_TZe_MQG35,
      0x60 => TapeColor::Yellow,
      0x61 => TapeColor::Pink,
      0x62 => TapeColor::Blue,
      0x70 => TapeColor::HeatShrinkTube,
      0x90 => TapeColor::WhiteFlexId,
      0x91 => TapeColor::YellowFlexId,
      0xf0 => TapeColor::Cleaning,
      0xf1 => TapeColor::Stencil,
      0xff => TapeColor::Incompatible,
      v => TapeColor::Unknown(v),
    }
  }
}

impl TapeColor {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, color) = u8(input)?;

    Ok((input, color.into()))
  }
}
