use bitvec::{order::Msb0, view::BitView};
use image::DynamicImage;

use crate::{Error, Printer};

pub struct RasterLine;

impl RasterLine {
  pub async fn message(printer: &Printer, image: DynamicImage) -> Result<(), Error> {
    let image = image.rotate90().into_luma8();

    let image = image::imageops::flip_horizontal(&image);

    let width = image.width();

    let max_width = printer.ty.info().max_px as usize;
    let offset = (max_width / 2) - (width as usize / 2);

    for row in image.rows() {
      let mut raster_line = vec![0u8; max_width / 8];
      let raster_line_view = raster_line.view_bits_mut::<Msb0>();

      for (i, pixel) in row.enumerate() {
        if pixel.0[0] > 127 {
          raster_line_view.set(i + offset, false);
        } else {
          raster_line_view.set(i + offset, true);
        }
      }

      printer
        .send(RasterLine::row_buffer(
          raster_line,
          printer.flags_contains(crate::PrinterFlags::RasterPackBits),
        ))
        .await?;
    }

    Ok(())
  }

  fn row_buffer(row: Vec<u8>, pack_bits: bool) -> Vec<u8> {
    if pack_bits {
      let mut buf = vec![0; row.len() + 4];
      buf[0] = 0x47;

      // Fake compression by encoding a single uncompressed run
      buf[1] = row.len() as u8 + 1;
      buf[2] = 0;
      buf[3] = row.len() as u8 - 1;

      buf[4..].copy_from_slice(&row);

      buf
    } else {
      let mut buf = vec![0; row.len() + 3];
      buf[0] = 0x47;

      buf[1] = row.len() as u8;
      buf[2] = 0;

      buf[3..].copy_from_slice(&row);

      buf
    }
  }
}
