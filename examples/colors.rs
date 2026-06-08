use escpos::{driver::UsbDriver, printer::Printer, utils::Protocol};
use eyre::Result;

/// `ESC r n` — select print color on the two-color ribbon.
/// n = 0 -> black (upper part of ribbon), n = 1 -> red (lower part).
fn select_color(red: bool) -> [u8; 3] {
    [0x1b, 0x72, red as u8]
}

fn main() -> Result<()> {
    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;

    let mut printer = Printer::new(driver, Protocol::default(), None);
    printer.init()?;

    let lines = [
        "Black line one",
        "Red line two",
        "Black line three",
        "Red line four",
        "Black line five",
    ];

    for (i, line) in lines.iter().enumerate() {
        // Even lines black, odd lines red.
        printer.custom(&select_color(i % 2 == 1))?;
        printer.writeln(line)?;
    }

    // Leave the ribbon back on black for subsequent jobs.
    printer.custom(&select_color(false))?;
    printer.print_cut()?;

    Ok(())
}
