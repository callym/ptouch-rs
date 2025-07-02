use crate::{Error, Printer};

pub struct Finalize;

impl Finalize {
  pub async fn message(printer: &Printer, chain: bool) -> Result<(), Error> {
    if chain && !printer.flags_contains(crate::PrinterFlags::D460BTMagic) {
      printer.send([0x0c]).await
    } else {
      printer.send([0x1a]).await
    }
  }
}
