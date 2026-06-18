use eyre::{Result, eyre};

const PRINT_URL: &str = "http://localhost:8080/print";

pub fn print(text: &str) -> Result<()> {
    let response = minreq::post(PRINT_URL).with_body(text).send()?;

    match response.status_code {
        200..=299 => Ok(()),
        status => Err(eyre!(
            "print server returned {}: {}",
            status,
            response.as_str().unwrap_or("<non-text body>").trim()
        )),
    }
}
