use escpos::{
    driver::UsbDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    ui::line::{LineBuilder, LineStyle},
    utils::{DebugMode, JustifyMode, PageCode, Protocol, UnderlineMode},
};
use eyre::Result;

fn main() -> Result<()> {
    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;

    let printer_options = PrinterOptions::new(Some(PageCode::PC437), Some(DebugMode::Dec), 40);
    let mut printer = Printer::new(driver, Protocol::default(), Some(printer_options));
    printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .bold(true)?
        .underline(UnderlineMode::Single)?
        .writeln("Bold underline left")?
        .justify(JustifyMode::CENTER)?
        .reverse(false)?
        .bold(false)?
        .writeln("Normal underline")?
        .feed()?
        .justify(JustifyMode::RIGHT)?
        .underline(UnderlineMode::None)?
        .draw_line(LineBuilder::new().style(LineStyle::Double).build())?
        .feed()?
        .writeln("Normal right")?
        .feed()?
        .print_cut()?;

    Ok(())
}
