//! Print one or more lines of text rotated 90° so they run lengthwise down the
//! paper strip, allowing banners of arbitrary length.
//!
//! The lines are stacked and centered with [`render_lines_centered`] (a wide,
//! short bitmap), then rotated a quarter turn with [`Bitmap::rotated_ccw`].
//! After rotation each line becomes its own column running the length of the
//! strip, centered across its width; the text length becomes the feed length,
//! so longer strings print a longer strip.
//!
//! The font is automatically scaled down so all the lines fit across the
//! printable width: one or two lines print at the desired size, and adding more
//! lines shrinks the glyphs (preserving their aspect ratio) until they fit.

use eyre::Result;
use print_server::graphics::print_bitmap;
use print_server::text::{TextStyle, fit_lines, render_lines_centered};

/// `ESC *` mode (0 = single density ~60 dpi, 1 = double density ~120 dpi).
const MODE: u8 = 0;
/// Per-band line feed pitch in 1/144" units (see the graphics example).
const BAND_FEED: u8 = 16;

/// Printable width across the paper, in dots. The output is centered within
/// this, so it should match the printer's actual printable width.
const PRINT_WIDTH: usize = 200;

/// Desired (maximum) glyph scale along the length of the paper.
const SCALE_X: usize = 6;
/// Desired (maximum) glyph scale across the width of the paper.
const SCALE_Y: usize = 8;
/// Gap between lines, in font pixels (scaled with the glyphs).
const LINE_GAP_PX: usize = 2;
/// Trim each glyph to its actual ink width so characters sit closer together.
const PROPORTIONAL: bool = true;

/// The banner lines, centered as a block. Add as many as you like; the glyphs
/// shrink to keep all lines on the paper.
const LINES: &[&str] = &["First", "Second", "Third", "Fourth"];

fn main() -> Result<()> {
    let desired = TextStyle {
        scale_x: SCALE_X,
        scale_y: SCALE_Y,
        letter_spacing: SCALE_X,
        proportional: PROPORTIONAL,
    };
    let (style, line_gap) = fit_lines(LINES.len(), PRINT_WIDTH, &desired, LINE_GAP_PX);
    let block = render_lines_centered(LINES, PRINT_WIDTH, line_gap, &style);
    let banner = block.rotated_ccw();
    print_bitmap(&banner, MODE, BAND_FEED)?;
    Ok(())
}
