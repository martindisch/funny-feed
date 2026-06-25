//! Manual `ESC *` bit-image graphics on a 9-pin TM-U220.
//!
//! Builds a 1-bit test image and prints it as a contiguous bit image via the
//! reusable helpers in [`demo::graphics`]. See that module for why we drive
//! `ESC *` directly instead of using the crate's `bit_image*` (`GS v 0`) path.

use demo::graphics::{Bitmap, print_bitmap};
use eyre::Result;

/// Image width in dot columns.
const WIDTH: usize = 192;
/// Image height in dot rows.
const HEIGHT: usize = 96;
/// `ESC *` mode. 0 = 8-dot single density (~60 dpi), 1 = 8-dot double density
/// (~120 dpi). The TM-U220 supports modes 0 and 1 only.
const MODE: u8 = 0;
/// Line feed pitch per band, in 1/144" units. If bands show white stripes,
/// increase it; if they overlap, decrease it. `16` matches the U220's 72 dpi
/// vertical pitch (8 dots = 8/72", and `ESC 3` uses 1/144" units).
const BAND_FEED: u8 = 16;

fn main() -> Result<()> {
    let bitmap = render_bitmap();
    print_bitmap(&bitmap, MODE, BAND_FEED)?;
    Ok(())
}

/// Build a 1-bit test image that makes contiguity easy to judge: an outer
/// border, a solid filled disk (any banding would show as white stripes across
/// it), and a diagonal cross.
fn render_bitmap() -> Bitmap {
    let mut bmp = Bitmap::new(WIDTH, HEIGHT);

    // Outer border.
    for x in 0..WIDTH as isize {
        bmp.set(x, 0);
        bmp.set(x, HEIGHT as isize - 1);
    }
    for y in 0..HEIGHT as isize {
        bmp.set(0, y);
        bmp.set(WIDTH as isize - 1, y);
    }

    // Solid filled disk on the left, centered vertically.
    let (cx, cy) = (HEIGHT as isize / 2, HEIGHT as isize / 2);
    let r = HEIGHT as isize / 2 - 8;
    for y in -r..=r {
        for x in -r..=r {
            if x * x + y * y <= r * r {
                bmp.set(cx + x, cy + y);
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
        bmp.set(x0 + i, y);
        bmp.set(x0 + i, HEIGHT as isize - 1 - y);
    }

    bmp
}
