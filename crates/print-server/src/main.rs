use escpos::{
    driver::UsbDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{PageCode, Protocol},
};
use eyre::{Result, eyre};
use tiny_http::{Method, Request, Response, Server};

mod food;

const VENDOR_ID: u16 = 0x04b8;
const PRODUCT_ID: u16 = 0x0202;

const ADDR: &str = "127.0.0.1:8080";

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
            (Method::Get, "/food") => match food::handle_food(&mut printer) {
                Ok(()) => Response::from_string("printed\n"),
                Err(e) => {
                    eprintln!("food error: {e:?}");
                    Response::from_string(format!("food failed: {e}\n")).with_status_code(500)
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
