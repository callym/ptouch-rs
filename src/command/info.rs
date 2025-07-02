use crate::{Error, Printer, PrinterFlags};

pub struct Info;

impl Info {
  pub async fn message(printer: &Printer, size_x: u32) -> Result<(), Error> {
    let mut message = [
      0x1b, 0x69, 0x71, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    message[5] = printer.status.media_width.into();

    for (i, byte) in size_x.to_le_bytes().into_iter().enumerate() {
      message[7 + i] = byte;
    }

    if printer.flags_contains(PrinterFlags::D460BTMagic) {
      message[11] = 0x02;
    }

    printer.send(message).await
  }
}
