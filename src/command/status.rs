use nom::{Finish, IResult, Parser, number::complete::u8};

use crate::{
  Error,
  MediaType,
  TapeColor,
  TapeSize,
  TextColor,
  nom_utils::{tag, tag_multi, u16, u32, zero, zero_multi},
  status_type::StatusType,
};

fn parse(input: &[u8]) -> IResult<&[u8], Status> {
  let (input, _) = tag_multi([0x80, 0x20, b'B', b'0'])(input)?;

  let (input, model) = u8(input)?;

  let (input, _) = tag(b'0')(input)?;
  let (input, _) = u16(input)?;

  let (input, error) = u16(input)?;

  let (input, media_width) = TapeSize::parse(input)?;

  let (input, media_type) = MediaType::parse(input)?;

  let (input, _) = zero_multi::<3>(input)?;
  let (input, mode) = u8(input)?;

  let (input, _) = zero_multi::<2>(input)?;

  let (input, status_type) = StatusType::parse(input)?;
  let (input, phase_type) = u8(input)?;
  let (input, phase_number) = u16(input)?;
  let (input, notification_number) = u8(input)?;

  let (input, _) = zero(input)?;

  let (input, tape_color) = TapeColor::parse(input)?;
  let (input, text_color) = TextColor::parse(input)?;

  let (input, hw_setting) = u32(input)?;

  nom::combinator::all_consuming(u16).parse(input)?;

  Ok((
    &[],
    Status {
      model,
      error,
      media_width,
      media_type,
      mode,
      status_type,
      phase_type,
      phase_number,
      notification_number,
      tape_color,
      text_color,
      hw_setting,
    },
  ))
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Status {
  pub model: u8,
  pub error: u16,
  pub media_width: TapeSize,
  pub media_type: MediaType,
  pub mode: u8,
  pub status_type: StatusType,
  pub phase_type: u8,
  pub phase_number: u16,
  pub notification_number: u8,
  pub tape_color: TapeColor,
  pub text_color: TextColor,
  pub hw_setting: u32,
}

impl Status {
  pub const fn message() -> &'static [u8] {
    b"\x1biS"
  }

  pub fn from_request(buf: Vec<u8>) -> Result<Self, Error> {
    match parse(&buf).finish() {
      Ok((_, status)) => Ok(status),
      Err(nom::error::Error { code, input }) => Err(nom::Err::Error(nom::error::Error {
        code,
        input: input.to_vec(),
      }))?,
    }
  }
}
