//! Print one or more lines of text rotated 90° so they run lengthwise down the
//! paper strip, allowing banners of arbitrary length.
//!
//! The lines are rendered horizontally and stacked with
//! [`demo::text::render_lines`] (a wide, short bitmap), then rotated a quarter
//! turn with [`Bitmap::rotated_ccw`]. After rotation each line becomes its own
//! column running the length of the strip, side by side across its width; the
//! text length becomes the feed length, so longer strings print a longer strip.
//!
//! Because rotation swaps the axes, `SCALE_X` ends up running along the paper
//! (printed at ~72 dpi) and `SCALE_Y` runs across it (~60 dpi in `MODE` 0). Each
//! line takes ~`8 * SCALE_Y` dots of width, so watch the total against the
//! printable width when adding lines. Switch [`Bitmap::rotated_ccw`] to
//! [`Bitmap::rotated_cw`] to flip the reading direction.

use demo::graphics::print_bitmap;
use demo::text::{TextStyle, render_lines};
use eyre::Result;

/// `ESC *` mode (0 = single density ~60 dpi, 1 = double density ~120 dpi).
const MODE: u8 = 0;
/// Per-band line feed pitch in 1/144" units (see the graphics example).
const BAND_FEED: u8 = 16;

/// Glyph scale along the length of the paper.
const SCALE_X: usize = 6;
/// Glyph scale across the width of the paper.
const SCALE_Y: usize = 8;
/// Blank dots between adjacent glyphs. The embedded font already leaves a blank
/// column or two inside each 8px cell, so a small value here reads fine; set it
/// to 0 for the tightest the fixed-width cell allows.
const LETTER_SPACING: usize = 0;
/// Blank dots between adjacent lines (measured across the paper width).
const LINE_GAP: usize = SCALE_Y * 2;

/// The banner lines. Each runs the full length of the strip, side by side. Make
/// them as long as you like; the strip grows to fit the longest one.
const LINES: &[&str] = &["First", "Second"];

fn main() -> Result<()> {
    let style = TextStyle {
        scale_x: SCALE_X,
        scale_y: SCALE_Y,
        letter_spacing: LETTER_SPACING,
    };
    let block = render_lines(LINES, LINE_GAP, &style);
    let banner = block.rotated_ccw();
    print_bitmap(&banner, MODE, BAND_FEED)?;
    Ok(())
}
