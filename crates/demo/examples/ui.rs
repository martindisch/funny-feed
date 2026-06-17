use escpos::{
    driver::UsbDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    ui::line::{LineBuilder, LineStyle},
    utils::{DebugMode, JustifyMode, PageCode, Protocol},
};
use eyre::Result;

fn main() -> Result<()> {
    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;

    let line_double = LineBuilder::new().style(LineStyle::Double).build();
    let line_simple = LineBuilder::new()
        .style(LineStyle::Simple)
        .offset(4)
        .build();
    let line_dotted = LineBuilder::new()
        .style(LineStyle::Dotted)
        .offset(8)
        .justify(JustifyMode::RIGHT)
        .build();
    let line_dashed = LineBuilder::new()
        .style(LineStyle::Dashed)
        .justify(JustifyMode::CENTER)
        .size((2, 1))
        .width(8)
        .build();
    let line_custom = LineBuilder::new().style(LineStyle::Custom("┼")).build();

    let printer_options = PrinterOptions::new(Some(PageCode::PC437), Some(DebugMode::Dec), 40);
    let mut printer = Printer::new(driver, Protocol::default(), Some(printer_options));
    printer
        .init()?
        .writeln("UI Components")?
        .feed()?
        .writeln("Lines")?
        .draw_line(line_double)?
        .draw_line(line_simple)?
        .draw_line(line_dotted)?
        .draw_line(line_dashed)?
        .draw_line(line_custom)?
        .print_cut()?;

    Ok(())
}
