//! `/banner` endpoint: render the request body as large rotated text running
//! lengthwise down the paper strip.
//!
//! The body is read as plain text, one banner line per input line. Lines are
//! centered as a block across the strip and the glyphs auto-shrink so that all
//! lines fit within the printable width.

use escpos::{driver::UsbDriver, printer::Printer};
use eyre::{Result, eyre};
use tiny_http::Request;

use print_server::graphics::write_bitmap;
use print_server::text::{TextStyle, fit_lines, render_lines_centered};

/// Printable width across the paper, in dots. Output is centered within this, so
/// it should match the printer's actual printable width.
const PRINT_WIDTH: usize = 200;
/// `ESC *` mode (0 = single density ~60 dpi, 1 = double density ~120 dpi).
const MODE: u8 = 0;
/// Per-band line feed pitch in 1/144" units.
const BAND_FEED: u8 = 16;
/// Desired (maximum) glyph scale along the length of the paper.
const SCALE_X: usize = 6;
/// Desired (maximum) glyph scale across the width of the paper.
const SCALE_Y: usize = 8;
/// Gap between lines, in font pixels (scaled with the glyphs).
const LINE_GAP_PX: usize = 2;
/// Trim each glyph to its actual ink width so characters sit closer together.
const PROPORTIONAL: bool = true;

pub fn handle_banner(request: &mut Request, printer: &mut Printer<UsbDriver>) -> Result<()> {
    let mut body = String::new();
    request.as_reader().read_to_string(&mut body)?;

    let lines: Vec<&str> = body
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
        .collect();
    if lines.is_empty() {
        return Err(eyre!("banner requires at least one non-empty line"));
    }

    println!("banner job with {} line(s)", lines.len());
    print_banner(printer, &lines)
}

fn print_banner(printer: &mut Printer<UsbDriver>, lines: &[&str]) -> Result<()> {
    let desired = TextStyle {
        scale_x: SCALE_X,
        scale_y: SCALE_Y,
        letter_spacing: SCALE_X,
        proportional: PROPORTIONAL,
    };
    let (style, line_gap) = fit_lines(lines.len(), PRINT_WIDTH, &desired, LINE_GAP_PX);
    let block = render_lines_centered(lines, PRINT_WIDTH, line_gap, &style);
    let banner = block.rotated_ccw();

    printer.init()?;
    write_bitmap(printer, &banner, MODE, BAND_FEED)?;
    printer.print_cut()?;

    Ok(())
}
