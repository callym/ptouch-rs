use nom::{IResult, number::Endianness};

pub fn tag<'a>(byte: u8) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ()> {
  move |input| {
    let byte = &[byte][..];
    let (input, _) = nom::bytes::complete::tag(byte)(input)?;

    Ok((input, ()))
  }
}

pub fn tag_multi<'a>(byte: impl AsRef<[u8]>) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ()> {
  move |input| {
    let (input, _) = nom::bytes::complete::tag(byte.as_ref())(input)?;

    Ok((input, ()))
  }
}

pub fn zero(input: &[u8]) -> IResult<&[u8], ()> {
  tag(0)(input)
}

pub fn zero_multi<const N: usize>(input: &[u8]) -> IResult<&[u8], ()> {
  tag_multi([0; N])(input)
}

pub fn u16(input: &[u8]) -> IResult<&[u8], u16> {
  nom::number::complete::u16(Endianness::Native)(input)
}

pub fn u32(input: &[u8]) -> IResult<&[u8], u32> {
  nom::number::complete::u32(Endianness::Native)(input)
}
