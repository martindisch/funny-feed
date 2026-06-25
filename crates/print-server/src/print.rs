use escpos::{driver::UsbDriver, printer::Printer};
use eyre::Result;
use tiny_http::Request;

pub fn handle_print(request: &mut Request, printer: &mut Printer<UsbDriver>) -> Result<()> {
    let sender = sender_ip(request);
    println!("print job from {sender}");
    let mut body = String::new();
    request.as_reader().read_to_string(&mut body)?;
    print_text(printer, &body)
}

fn sender_ip(request: &Request) -> String {
    request
        .headers()
        .iter()
        .find(|header| header.field.equiv("X-Real-IP"))
        .map(|header| header.value.as_str().to_string())
        .or_else(|| request.remote_addr().map(|addr| addr.ip().to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}

fn print_text(printer: &mut Printer<UsbDriver>, text: &str) -> Result<()> {
    let mut p = printer.init()?;
    for line in text.lines() {
        p = p.writeln(line)?;
    }
    p.print_cut()?;

    Ok(())
}
