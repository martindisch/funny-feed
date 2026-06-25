//! Print a single line of text rotated 90° so it runs lengthwise down the paper
//! strip, allowing banners of arbitrary length.
//!
//! The line is rendered horizontally with [`demo::text::render_line`] (a wide,
//! short bitmap), then rotated a quarter turn with [`Bitmap::rotated_ccw`]. The
//! glyph height becomes the printed width across the paper (~0.5" here), and the
//! text length becomes the feed length, so longer strings simply print a longer
//! strip.
//!
//! Because rotation swaps the axes, `SCALE_X` ends up running along the paper
//! (printed at ~72 dpi) and `SCALE_Y` runs across it (~60 dpi in `MODE` 0).
//! Switch [`Bitmap::rotated_ccw`] to [`Bitmap::rotated_cw`] to flip the reading
//! direction.

use demo::graphics::print_bitmap;
use demo::text::{TextStyle, render_line};
use eyre::Result;

/// `ESC *` mode (0 = single density ~60 dpi, 1 = double density ~120 dpi).
const MODE: u8 = 0;
/// Per-band line feed pitch in 1/144" units (see the graphics example).
const BAND_FEED: u8 = 16;

/// Glyph scale along the length of the paper.
const SCALE_X: usize = 6;
/// Glyph scale across the width of the paper.
const SCALE_Y: usize = 8;
/// Blank dots between adjacent glyphs.
const LETTER_SPACING: usize = SCALE_X;

/// The banner text. Make it as long as you like; the strip grows to fit.
const TEXT: &str = "Banner";

fn main() -> Result<()> {
    let style = TextStyle {
        scale_x: SCALE_X,
        scale_y: SCALE_Y,
        letter_spacing: LETTER_SPACING,
    };
    let line = render_line(TEXT, &style);
    let banner = line.rotated_ccw();
    print_bitmap(&banner, MODE, BAND_FEED)?;
    Ok(())
}
