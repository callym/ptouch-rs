use crate::{Error, Printer};

pub struct Precut;

impl Precut {
  pub async fn message(printer: &Printer, precut: bool) -> Result<(), Error> {
    printer
      .send([0x1b, 0x69, 0x4d, if precut { 0x40 } else { 0x00 }])
      .await
  }
}
