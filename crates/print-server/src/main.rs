use escpos::{
    driver::UsbDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{PageCode, Protocol},
};
use eyre::{Result, eyre};
use tiny_http::{Method, Request, Response, Server};

const VENDOR_ID: u16 = 0x04b8;
const PRODUCT_ID: u16 = 0x0202;

const ADDR: &str = "0.0.0.0:8080";

fn main() -> Result<()> {
    let driver = UsbDriver::open(VENDOR_ID, PRODUCT_ID, None, None)?;
    let mut printer = Printer::new(
        driver,
        Protocol::default(),
        Some(PrinterOptions::new(Some(PageCode::PC850), None, 40)),
    );

    let server = Server::http(ADDR).map_err(|e| eyre!("failed to bind {ADDR}: {e}"))?;
    println!("print-server listening on http://{ADDR}");

    for mut request in server.incoming_requests() {
        let response = match (request.method(), request.url()) {
            (Method::Post, "/print") => match handle_print(&mut request, &mut printer) {
                Ok(()) => Response::from_string("printed\n"),
                Err(e) => {
                    eprintln!("print error: {e:?}");
                    Response::from_string(format!("print failed: {e}\n")).with_status_code(500)
                }
            },
            _ => Response::from_string("not found\n").with_status_code(404),
        };

        if let Err(e) = request.respond(response) {
            eprintln!("failed to send response: {e}");
        }
    }

    Ok(())
}

fn handle_print(request: &mut Request, printer: &mut Printer<UsbDriver>) -> Result<()> {
    let mut body = String::new();
    request.as_reader().read_to_string(&mut body)?;
    print_text(printer, &body)
}

fn print_text(printer: &mut Printer<UsbDriver>, text: &str) -> Result<()> {
    let mut p = printer.init()?;
    for line in text.lines() {
        p = p.writeln(line)?;
    }
    p.print_cut()?;

    Ok(())
}
