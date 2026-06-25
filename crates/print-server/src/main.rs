use escpos::{
    driver::UsbDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{PageCode, Protocol},
};
use eyre::{Result, eyre};
use tiny_http::{Method, Response, Server};

mod food;
mod print;

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
            (Method::Post, "/print") => match print::handle_print(&mut request, &mut printer) {
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
