use escpos::{
    driver::UsbDriver,
    printer::Printer,
    utils::{DebugMode, Protocol},
};
use eyre::Result;

fn main() -> Result<()> {
    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;

    Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("USB test")?
        .print_cut()?;

    Ok(())
}
