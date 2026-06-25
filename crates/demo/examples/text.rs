//! Print text as a scaled bitmap on the TM-U220.
//!
//! Renders an embedded 8x8 pixel font (see [`demo::text`]) into a [`Bitmap`] and
//! prints it via the `ESC *` bit-image path, so characters can be scaled far
//! larger than the printer's built-in text modes allow.
//!
//! Note the printed pixels are not square: vertical is ~72 dpi while horizontal
//! is ~60 dpi in `MODE` 0, so equal `SCALE_X`/`SCALE_Y` print slightly wider
//! than tall. Lower `SCALE_X` or switch to `MODE` 1 for squarer characters.

use eyre::Result;
use print_server::graphics::print_bitmap;
use print_server::text::{TextStyle, render_wrapped};

/// Printable width in dot columns. The TM-U220's printable area is only ~1.5"
/// wide, so large characters fit just a few per line.
const PRINT_WIDTH: usize = 200;
/// `ESC *` mode (0 = single density ~60 dpi, 1 = double density ~120 dpi).
const MODE: u8 = 0;
/// Per-band line feed pitch in 1/144" units (see the graphics example).
const BAND_FEED: u8 = 16;

/// Horizontal glyph scale factor (each font pixel becomes `SCALE_X` dots wide).
const SCALE_X: usize = 4;
/// Vertical glyph scale factor (each font pixel becomes `SCALE_Y` dots tall).
const SCALE_Y: usize = 4;
/// Blank dots between adjacent glyphs.
const LETTER_SPACING: usize = SCALE_X;
/// Trim glyphs to their ink width for tighter spacing. Off here keeps the
/// fixed-width grid look; set to `true` for proportional spacing.
const PROPORTIONAL: bool = false;
/// Blank dots between wrapped text lines.
const LINE_GAP: usize = SCALE_Y * 2;

/// The string to print. `\n` forces a line break; long lines wrap on character
/// boundaries at `PRINT_WIDTH`.
const TEXT: &str = "Hello\nworld";

fn main() -> Result<()> {
    let style = TextStyle {
        scale_x: SCALE_X,
        scale_y: SCALE_Y,
        letter_spacing: LETTER_SPACING,
        proportional: PROPORTIONAL,
    };
    let bitmap = render_wrapped(TEXT, PRINT_WIDTH, LINE_GAP, &style);
    print_bitmap(&bitmap, MODE, BAND_FEED)?;
    Ok(())
}
