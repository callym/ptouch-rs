mod tape_stats;
use nusb::{Interface, transfer::RequestBuffer};
pub use tape_stats::{TapeInfo, TapeSize};

mod command;
mod media_type;
pub(crate) mod nom_utils;
mod printer_stats;
mod tape_color;
mod text_color;

pub use command::{Commands, Status};
pub use media_type::MediaType;
pub use printer_stats::{PrinterFlags, PrinterInfo, PrinterType};
pub use tape_color::TapeColor;
pub use text_color::TextColor;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Nusb(#[from] nusb::Error),
  #[error(transparent)]
  NusbTransfer(#[from] nusb::transfer::TransferError),
  #[error(transparent)]
  Nom(#[from] nom::Err<nom::error::Error<Vec<u8>>>),
  #[error("Printer not found")]
  PrinterNotFound,
  #[error("Invalid tape size reported: {0}")]
  InvalidTapeSize(u8),
}

pub struct Printer {
  interface: PrinterInterface,
  status: Status,
  ty: PrinterType,
}

struct PrinterInterface {
  interface: Interface,
}

impl PrinterInterface {
  async fn send(&self, data: impl Into<Vec<u8>>) -> Result<(), Error> {
    self.interface.bulk_out(0x02, data.into()).await.status?;

    Ok(())
  }

  async fn receive(&self, data: impl Into<Vec<u8>>) -> Result<Vec<u8>, Error> {
    self.send(data.into()).await?;

    let buf = RequestBuffer::new(32);
    let res = self.interface.bulk_in(0x81, buf).await.into_result()?;

    Ok(res)
  }
}

impl Printer {
  pub async fn open() -> Result<Self, Error> {
    let found = nusb::list_devices()?.find_map(|device| {
      let printer = PrinterType::from_usb(device.vendor_id(), device.product_id())?;

      Some((device, printer))
    });

    let (device, ty) = match found {
      Some(ty) => ty,
      None => Err(Error::PrinterNotFound)?,
    };

    let device = device.open()?;
    let interface = device.detach_and_claim_interface(0)?;

    let interface = PrinterInterface { interface };

    let mut init = vec![0; 102];
    init[100] = 0x1b;
    init[101] = 0x40;
    interface.send(init).await?;

    let status = Commands::status(&interface).await?;

    Ok(Printer {
      interface,
      ty,
      status,
    })
  }

  fn flags_contains(&self, flag: PrinterFlags) -> bool {
    self.ty.info().flags.contains(flag)
  }

  async fn send(&self, data: impl Into<Vec<u8>>) -> Result<(), Error> {
    self.interface.send(data).await
  }

  pub async fn print(&self, image: image::DynamicImage) -> Result<(), Error> {
    if self.flags_contains(PrinterFlags::RasterPackBits) {
      Commands::pack_bits(self).await?;
    }

    Commands::raster_start(self).await?;

    if self.flags_contains(PrinterFlags::UseInfoCmd) {
      Commands::info(self, image.width()).await?;
    }

    if self.flags_contains(PrinterFlags::D460BTMagic) {
      Commands::d460bt_magic(self, false).await?;
    }

    if self.flags_contains(PrinterFlags::HasPrecut) {
      Commands::precut(self, true).await?;
    }

    Commands::raster_line(self, image).await?;

    Commands::finalize(self, false).await?;

    Ok(())
  }
}
