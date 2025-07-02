use crate::{Error, Printer, PrinterInterface};

mod d460bt;
mod finalize;
mod info;
mod packbits;
mod precut;
mod raster_start;
mod rasterline;
mod status;

use d460bt::D490bt;
use finalize::Finalize;
use image::DynamicImage;
use info::Info;
use packbits::PackBits;
use precut::Precut;
use raster_start::RasterStart;
use rasterline::RasterLine;
pub use status::Status;

pub struct Commands;

impl Commands {
  pub async fn pack_bits(printer: &Printer) -> Result<(), Error> {
    printer.send(PackBits::message()).await
  }

  pub(crate) async fn status(printer: &PrinterInterface) -> Result<Status, Error> {
    let buf = printer.receive(Status::message()).await?;
    let status = Status::from_request(buf)?;

    Ok(status)
  }

  pub async fn info(printer: &Printer, size_x: u32) -> Result<(), Error> {
    Info::message(printer, size_x).await
  }

  pub async fn raster_start(printer: &Printer) -> Result<(), Error> {
    RasterStart::message(printer).await
  }

  pub async fn d460bt_magic(printer: &Printer, chain: bool) -> Result<(), Error> {
    D490bt::message(printer, chain).await
  }

  pub async fn precut(printer: &Printer, precut: bool) -> Result<(), Error> {
    Precut::message(printer, precut).await
  }

  pub async fn raster_line(printer: &Printer, image: DynamicImage) -> Result<(), Error> {
    RasterLine::message(printer, image).await
  }

  pub async fn finalize(printer: &Printer, chain: bool) -> Result<(), Error> {
    Finalize::message(printer, chain).await
  }
}
