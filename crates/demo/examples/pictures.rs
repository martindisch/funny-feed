use escpos::{
    driver::UsbDriver,
    printer::Printer,
    utils::{JustifyMode, Protocol},
};
use eyre::Result;

/// ATTENTION: this doesn't work on the TM-U220 because it doesn't implement
/// the GS v 0 command.
fn main() -> Result<()> {
    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;

    let repo_root_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let mut printer = Printer::new(driver, Protocol::default(), None);
    printer
        .init()?
        .justify(JustifyMode::CENTER)?
        .bit_image(&(repo_root_dir + "/img/rust-logo-small.png"))?
        .feed()?
        .print_cut()?;

    Ok(())
}
