use std::{collections::HashMap, sync::LazyLock};

use enumflags2::{BitFlags, bitflags};

static USB_TO_DEVICE: LazyLock<HashMap<(u16, u16), PrinterType>> = LazyLock::new(|| {
  let mut map = HashMap::new();

  for (printer, info) in PrinterType::iter_info() {
    map.insert((info.vendor_id, info.product_id), *printer);
  }

  map
});

static INFO: LazyLock<HashMap<PrinterType, PrinterInfo>> = LazyLock::new(|| {
  let mut map = HashMap::new();

  for printer in PrinterType::iter() {
    map.insert(printer, printer.info());
  }

  map
});

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrinterFlags {
  UnsupportedRaster = (1 << 0),
  RasterPackBits = (1 << 1),
  PLite = (1 << 2),
  P700Init = (1 << 3),
  UseInfoCmd = (1 << 4),
  HasPrecut = (1 << 5),
  D460BTMagic = (1 << 6),
}

#[derive(Debug, Clone)]
pub struct PrinterInfo {
  pub vendor_id: u16,
  pub product_id: u16,
  pub max_px: u32,
  pub dpi: u32,
  pub flags: BitFlags<PrinterFlags>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum PrinterType {
  PT_9200DX,
  PT_2300,
  PT_2420PC,
  PT_2450PC,
  PT_1950,
  PT_2700,
  /// Notes about the PT-1230PC: While it is true that this printer supports max 12mm tapes, it apparently expects > 76px data - the first 32px must be blank.
  PT_1230PC,
  PT_2430PC,
  PT_1230PC_PLite,
  PT_2430PC_PLite,
  /// Notes about the PT-2730: was reported to need 48px whitespace within png-images before content is actually printed - can not check this
  PT_2730,
  /// Note about the PT-E500: was reported by Jesse Becker with the remark that it also needs some padding (white pixels)
  PT_H500,
  /// Note about the PT-E500: was reported by Jesse Becker with the remark that it also needs some padding (white pixels)
  PT_E500,
  PT_P700,
  PT_P750W,
  PT_P700_PLite,
  PT_P750W_PLite,
  PT_D410,
  /// Notes about the PT-D450: I'm unsure if print width really is 128px
  PT_D450,
  PT_D460BT,
  /// PT-D600 was reported to work, but with some quirks (premature cutting of tape, printing maximum of 73mm length)
  PT_D600,
  PT_D610BT,
  PT_P710BT,
  /// added by Christian, PT-E310BT (aka PT-E310BTVP) requires these flags, otherwise not returning from libusb_bulk_transfer-call
  /// printhead 128px, 180 dpi resolution
  /// 3,5/6/9/12/18 mm TZe Tapes, 12mm and 18mm tested
  /// 5,2/9/11,2 mm HSe heat shrink tubes not tested, probably requiring extension of struct _pt_tape_info
  PT_E310BT,
}

impl PrinterType {
  pub fn from_usb(vendor: u16, product: u16) -> Option<Self> {
    USB_TO_DEVICE.get(&(vendor, product)).cloned()
  }

  pub fn iter() -> impl Iterator<Item = Self> {
    <Self as strum::IntoEnumIterator>::iter()
  }

  pub fn iter_info() -> impl Iterator<Item = (&'static Self, &'static PrinterInfo)> {
    INFO.iter()
  }

  pub fn info(&self) -> PrinterInfo {
    match self {
      PrinterType::PT_9200DX => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2001,
        max_px: 384,
        dpi: 360,
        flags: PrinterFlags::RasterPackBits | PrinterFlags::HasPrecut,
      },
      PrinterType::PT_2300 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2004,
        max_px: 112,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits | PrinterFlags::HasPrecut,
      },
      PrinterType::PT_2420PC => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2007,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits.into(),
      },
      PrinterType::PT_2450PC => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2011,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits.into(),
      },
      PrinterType::PT_1950 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2019,
        max_px: 112,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits.into(),
      },
      PrinterType::PT_2700 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x201f,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::HasPrecut.into(),
      },
      PrinterType::PT_1230PC => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x202c,
        max_px: 128,
        dpi: 180,
        flags: BitFlags::empty(),
      },
      PrinterType::PT_2430PC => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x202d,
        max_px: 128,
        dpi: 180,
        flags: BitFlags::empty(),
      },
      PrinterType::PT_1230PC_PLite => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2030,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::PLite.into(),
      },
      PrinterType::PT_2430PC_PLite => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2031,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::PLite.into(),
      },
      PrinterType::PT_2730 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2041,
        max_px: 128,
        dpi: 180,
        flags: BitFlags::empty(),
      },
      PrinterType::PT_H500 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x205e,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits.into(),
      },
      PrinterType::PT_E500 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x205f,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits.into(),
      },
      PrinterType::PT_P700 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2061,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits | PrinterFlags::P700Init | PrinterFlags::HasPrecut,
      },
      PrinterType::PT_P750W => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2062,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits | PrinterFlags::P700Init,
      },
      PrinterType::PT_P700_PLite => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2064,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::PLite.into(),
      },
      PrinterType::PT_P750W_PLite => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2065,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::PLite.into(),
      },
      PrinterType::PT_D410 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x20df,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::UseInfoCmd | PrinterFlags::HasPrecut | PrinterFlags::D460BTMagic,
      },
      PrinterType::PT_D450 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2073,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::UseInfoCmd.into(),
      },
      PrinterType::PT_D460BT => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x20e0,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::P700Init
          | PrinterFlags::UseInfoCmd
          | PrinterFlags::HasPrecut
          | PrinterFlags::D460BTMagic,
      },
      PrinterType::PT_D600 => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2074,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits.into(),
      },
      PrinterType::PT_D610BT => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x20e1,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::P700Init
          | PrinterFlags::UseInfoCmd
          | PrinterFlags::HasPrecut
          | PrinterFlags::D460BTMagic,
      },
      PrinterType::PT_P710BT => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x20af,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::RasterPackBits | PrinterFlags::HasPrecut,
      },
      PrinterType::PT_E310BT => PrinterInfo {
        vendor_id: 0x04f9,
        product_id: 0x2201,
        max_px: 128,
        dpi: 180,
        flags: PrinterFlags::P700Init | PrinterFlags::UseInfoCmd | PrinterFlags::D460BTMagic,
      },
    }
  }
}
