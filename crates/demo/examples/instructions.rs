use escpos::{
    driver::UsbDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{PageCode, Protocol, UnderlineMode},
};
use eyre::Result;

fn main() -> Result<()> {
    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;

    let mut printer = Printer::new(
        driver,
        Protocol::default(),
        Some(PrinterOptions::new(Some(PageCode::PC850), None, 40)),
    );

    printer
        .init()?
        .bold(true)?
        .underline(UnderlineMode::Single)?
        .writeln("Instructions")?
        .bold(false)?
        .underline(UnderlineMode::None)?
        .writeln("Connect to LAN SOLO (WiFi)")?
        .feed()?
        .bold(true)?
        .writeln("Simple text")?
        .bold(false)?
        .writeln(
            "printf 'First line\\nSecond line' |\ncurl --data-binary @-\nhttp://hydraulic-pi.local/print",
        )?
        .feed()?
        .bold(true)?
        .writeln("Banner")?
        .bold(false)?
        .writeln(
            "printf 'First line\\nSecond line' |\ncurl --data-binary @-\nhttp://hydraulic-pi.local/banner",
        )?
        .print_cut()?;

    Ok(())
}
