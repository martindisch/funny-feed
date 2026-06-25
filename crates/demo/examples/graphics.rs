//! Manual `ESC *` bit-image graphics on a 9-pin TM-U220.
//!
//! The `bit_image*` helpers in the `escpos` crate emit the raster command
//! (`GS v 0`), which the TM-U220 does not support. This example instead drives
//! the legacy column bit-image command `ESC * m nL nH d1..dk` directly through
//! [`Printer::custom`], and stacks 8-dot-tall bands on top of each other so the
//! result is a single contiguous image with no horizontal gaps.
//!
//! The trick for "no holes" is to set the line feed pitch to exactly the height
//! of one band (8 dots) via `ESC 3 n` ([`Printer::line_spacing`]). If the bands
//! show white stripes between them, increase `BAND_FEED`; if they overlap,
//! decrease it. `16` matches the U220's documented 72 dpi vertical pitch
//! (8 dots = 8/72", and `ESC 3` uses 1/144" units, so 8 * 144/72 = 16).

use escpos::{
    driver::UsbDriver,
    printer::Printer,
    utils::{DebugMode, JustifyMode, Protocol},
};
use eyre::Result;

/// Image width in dot columns.
const WIDTH: usize = 192;
/// Image height in dot rows. Must be a multiple of 8 (one band = 8 rows).
const HEIGHT: usize = 96;
/// `ESC *` mode. 0 = 8-dot single density (~60 dpi), 1 = 8-dot double density
/// (~120 dpi). The TM-U220 supports modes 0 and 1 only.
const MODE: u8 = 0;
/// Line feed pitch per band, in 1/144" units. See module docs for tuning.
const BAND_FEED: u8 = 16;

fn main() -> Result<()> {
    let bitmap = render_bitmap();
    let payload = encode_bit_image(&bitmap);

    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;
    Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        // `ESC U 1`: unidirectional print mode. The head prints every band in
        // the same direction, avoiding the bidirectional registration backlash
        // that makes vertical lines (e.g. the right border) look jagged.
        .custom(&[0x1B, 0x55, 0x01])?
        .justify(JustifyMode::LEFT)?
        .line_spacing(BAND_FEED)?
        .custom(&payload)?
        .reset_line_spacing()?
        .feed()?
        .print_cut()?;

    Ok(())
}

/// Build a 1-bit test image (`true` == black dot) that makes contiguity easy to
/// judge: an outer border, a solid filled disk (any banding would show as white
/// stripes across it), and a diagonal cross.
fn render_bitmap() -> Vec<bool> {
    let mut px = vec![false; WIDTH * HEIGHT];
    let set = |x: isize, y: isize, px: &mut Vec<bool>| {
        if (0..WIDTH as isize).contains(&x) && (0..HEIGHT as isize).contains(&y) {
            px[y as usize * WIDTH + x as usize] = true;
        }
    };

    // Outer border.
    for x in 0..WIDTH as isize {
        set(x, 0, &mut px);
        set(x, HEIGHT as isize - 1, &mut px);
    }
    for y in 0..HEIGHT as isize {
        set(0, y, &mut px);
        set(WIDTH as isize - 1, y, &mut px);
    }

    // Solid filled disk on the left, centered vertically.
    let (cx, cy) = (HEIGHT as isize / 2, HEIGHT as isize / 2);
    let r = HEIGHT as isize / 2 - 8;
    for y in -r..=r {
        for x in -r..=r {
            if x * x + y * y <= r * r {
                set(cx + x, cy + y, &mut px);
            }
        }
    }

    // Diagonal cross over the right half.
    let x0 = HEIGHT as isize;
    let x1 = WIDTH as isize - 8;
    let span = x1 - x0;
    for i in 0..span {
        let t = i as f64 / span as f64;
        let y = (8.0 + t * (HEIGHT as f64 - 16.0)) as isize;
        set(x0 + i, y, &mut px);
        set(x0 + i, HEIGHT as isize - 1 - y, &mut px);
    }

    px
}

/// Encode the bitmap into a sequence of `ESC * m nL nH d..` bands, each followed
/// by a line feed (`0x0A`) that advances the paper by exactly one band height.
fn encode_bit_image(bitmap: &[bool]) -> Vec<u8> {
    let n_low = (WIDTH % 256) as u8;
    let n_high = (WIDTH / 256) as u8;
    let bands = HEIGHT / 8;

    let mut out = Vec::with_capacity(bands * (5 + WIDTH + 1));
    for band in 0..bands {
        out.extend_from_slice(&[0x1B, b'*', MODE, n_low, n_high]);
        for x in 0..WIDTH {
            let mut byte = 0u8;
            for bit in 0..8 {
                let y = band * 8 + bit;
                if bitmap[y * WIDTH + x] {
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
