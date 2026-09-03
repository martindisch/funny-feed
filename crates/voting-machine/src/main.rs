use std::io::Cursor;

use escpos::{
    driver::UsbDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{PageCode, Protocol},
};
use eyre::{Result, eyre};
use tiny_http::{Header, Method, Response, Server};

mod vote;

const VENDOR_ID: u16 = 0x04b8;
const PRODUCT_ID: u16 = 0x0202;

const ADDR: &str = "127.0.0.1:8080";

fn main() -> Result<()> {
    let driver = UsbDriver::open(VENDOR_ID, PRODUCT_ID, None, None)?;
    let mut printer = Printer::new(
        driver,
        Protocol::default(),
        Some(PrinterOptions::new(
            Some(PageCode::PC850),
            None,
            vote::LINE_WIDTH as u8,
        )),
    );

    let server = Server::http(ADDR).map_err(|e| eyre!("failed to bind {ADDR}: {e}"))?;
    println!("voting-machine listening on http://{ADDR}");

    for mut request in server.incoming_requests() {
        let response = match (request.method(), request.url()) {
            (Method::Get, "/vote") => html(vote::FORM_PAGE, 200),
            (Method::Post, "/vote") => match vote::handle_vote(&mut request, &mut printer) {
                Ok(vote::Outcome::Printed) => html(vote::THANKS_PAGE, 200),
                Ok(vote::Outcome::Empty) => html(vote::EMPTY_PAGE, 400),
                Err(e) => {
                    eprintln!("vote error: {e:?}");
                    html(vote::ERROR_PAGE, 500)
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

fn html(body: &str, status: u16) -> Response<Cursor<Vec<u8>>> {
    let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
        .expect("static content type header is valid");

    Response::from_string(body)
        .with_header(header)
        .with_status_code(status)
}
