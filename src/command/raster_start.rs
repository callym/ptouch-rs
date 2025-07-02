use crate::{Error, Printer, PrinterFlags};

pub struct RasterStart;

impl RasterStart {
  pub async fn message(printer: &Printer) -> Result<(), Error> {
    if printer.flags_contains(PrinterFlags::P700Init) {
      printer.send(b"\x1b\x69\x61\x01").await?;
    }

    printer.send(b"M\x02").await
  }
}
