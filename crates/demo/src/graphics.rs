//! Reusable 1-bit bitmap buffer and `ESC *` bit-image output for the TM-U220.
//!
//! The `bit_image*` helpers in the `escpos` crate emit the raster command
//! (`GS v 0`), which the TM-U220 does not support. We instead drive the legacy
//! column bit-image command `ESC * m nL nH d1..dk` directly and stack
//! 8-dot-tall bands so the result is a single contiguous image with no
//! horizontal gaps.

use escpos::{
    driver::UsbDriver,
    printer::Printer,
    utils::{DebugMode, JustifyMode, Protocol},
};
use eyre::Result;

/// A 1-bit image buffer where `true` means a black dot.
///
/// Pixels are stored row-major, one `bool` per dot.
pub struct Bitmap {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Bitmap {
    /// Create a blank (all-white) bitmap of the given dot dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    /// Width in dot columns.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height in dot rows.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Set a dot black. Coordinates outside the buffer are ignored, so callers
    /// can draw with signed math without bounds-checking every pixel.
    pub fn set(&mut self, x: isize, y: isize) {
        if (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y) {
            self.pixels[y as usize * self.width + x as usize] = true;
        }
    }

    /// Read a dot. Out-of-range coordinates read as white (`false`).
    pub fn get(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            false
        }
    }
}

/// Encode a bitmap into a sequence of `ESC * m nL nH d..` bands, each followed
/// by a line feed (`0x0A`) that advances the paper by exactly one band height.
///
/// `mode` selects the `ESC *` density (0 = 8-dot single density ~60 dpi,
/// 1 = 8-dot double density ~120 dpi); the TM-U220 supports modes 0 and 1.
/// The height is padded up to a whole number of 8-dot bands.
pub fn encode_bit_image(bitmap: &Bitmap, mode: u8) -> Vec<u8> {
    let width = bitmap.width();
    let n_low = (width % 256) as u8;
    let n_high = (width / 256) as u8;
    let bands = bitmap.height().div_ceil(8);

    let mut out = Vec::with_capacity(bands * (5 + width + 1));
    for band in 0..bands {
        out.extend_from_slice(&[0x1B, b'*', mode, n_low, n_high]);
        for x in 0..width {
            let mut byte = 0u8;
            for bit in 0..8 {
                let y = band * 8 + bit;
                if bitmap.get(x, y) {
                    // Bit 7 (MSB) is the top pin of the band.
                    byte |= 1 << (7 - bit);
                }
            }
            out.push(byte);
        }
        out.push(0x0A);
    }

    out
}

/// Open the USB printer and print a bitmap as a contiguous bit image.
///
/// `band_feed` is the per-band line feed pitch in 1/144" units (`ESC 3 n`);
/// `16` matches the U220's 72 dpi vertical pitch. Unidirectional print mode
/// (`ESC U 1`) is enabled so vertical lines stay crisp.
pub fn print_bitmap(bitmap: &Bitmap, mode: u8, band_feed: u8) -> Result<()> {
    let payload = encode_bit_image(bitmap, mode);

    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;
    Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        // `ESC U 1`: unidirectional print mode. The head prints every band in
        // the same direction, avoiding the bidirectional registration backlash
        // that makes vertical lines look jagged.
        .custom(&[0x1B, 0x55, 0x01])?
        .justify(JustifyMode::LEFT)?
        .line_spacing(band_feed)?
        .custom(&payload)?
        .reset_line_spacing()?
        .feed()?
        .print_cut()?;

    Ok(())
}
