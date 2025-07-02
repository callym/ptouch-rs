use crate::{Error, Printer};

pub struct D490bt;

impl D490bt {
  pub async fn message(printer: &Printer, chain: bool) -> Result<(), Error> {
    if chain {
      printer.send([0x1b, 0x69, 0x4b, 0x00]).await?;
    }

    // n1 and n2 are the length margin/spacing, in px? (uint16_t value, little endian)
    // A value of 0x06 is equivalent to the width margin on 6mm tape
    // The default for P-Touch software is 0x0e
    // n3 must be 0x4D or the print gets corrupted!
    // n4 seems to be ignored or reserved.
    printer
      .send([0x1b, 0x69, 0x64, 0x0e, 0x00, 0x49, 0x00])
      .await
  }
}
