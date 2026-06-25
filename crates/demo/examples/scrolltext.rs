//! Print one or more lines of text rotated 90° so they run lengthwise down the
//! paper strip, allowing banners of arbitrary length.
//!
//! The lines are stacked and centered with [`demo::text::render_lines_centered`]
//! (a wide, short bitmap), then rotated a quarter turn with
//! [`Bitmap::rotated_ccw`]. After rotation each line becomes its own column
//! running the length of the strip, centered across its width; the text length
//! becomes the feed length, so longer strings print a longer strip.
//!
//! The font is automatically scaled down so all the lines fit across the
//! printable width: one or two lines print at the desired size, and adding more
//! lines shrinks the glyphs (preserving their aspect ratio) until they fit.
//! Switch [`Bitmap::rotated_ccw`] to [`Bitmap::rotated_cw`] to flip the reading
//! direction.

use demo::graphics::print_bitmap;
use demo::text::{TextStyle, render_lines_centered, stack_height};
use eyre::Result;

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
    let (style, line_gap) = fit_to_strip(LINES.len());
    let block = render_lines_centered(LINES, PRINT_WIDTH, line_gap, &style);
    let banner = block.rotated_ccw();
    print_bitmap(&banner, MODE, BAND_FEED)?;
    Ok(())
}

/// Pick the largest glyph scale (down from the desired size) at which all
/// `n_lines` lines, plus the gaps between them, fit within [`PRINT_WIDTH`].
/// Returns the chosen style and the matching line gap in dots.
fn fit_to_strip(n_lines: usize) -> (TextStyle, usize) {
    let mut scale_y = SCALE_Y.max(1);
    loop {
        // Shrink horizontal scale proportionally to keep the glyph aspect ratio.
        let scale_x = (SCALE_X * scale_y / SCALE_Y).max(1);
        let line_gap = LINE_GAP_PX * scale_y;
        let style = TextStyle {
            scale_x,
            scale_y,
            letter_spacing: scale_x,
            proportional: PROPORTIONAL,
        };

        if scale_y == 1 || stack_height(n_lines, line_gap, &style) <= PRINT_WIDTH {
            return (style, line_gap);
        }
        scale_y -= 1;
    }
}
