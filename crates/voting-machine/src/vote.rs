use std::io::Read;

use escpos::{driver::UsbDriver, printer::Printer};
use eyre::Result;
use tiny_http::Request;

/// Characters per line the TM-U220 fits in its default font.
pub const LINE_WIDTH: usize = 40;
const MAX_BODY: u64 = 16 * 1024;
const MAX_SUGGESTION_CHARS: usize = 500;
const MAX_NAME_CHARS: usize = 67;

pub const FORM_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Voting machine</title>
</head>
<body>
<h1>Voting machine</h1>
<form method="post" action="/vote">
<p>
<label for="suggestions">Name suggestions</label><br>
<textarea id="suggestions" name="suggestions" rows="6" cols="24" required></textarea><br>
<small>One suggestion per line.</small>
</p>
<p>
<label for="name">Your name (optional)</label><br>
<input id="name" name="name" type="text" size="15" autocapitalize="words">
</p>
<button type="submit">Print</button>
</form>
</body>
</html>
"#;

pub const THANKS_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>Thanks</title></head>
<body><h1>Thanks!</h1><p>Your suggestions are on the paper.</p>
<p><a href="/vote">Vote again</a></p></body></html>
"#;

pub const EMPTY_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>Nothing to print</title></head>
<body><h1>Nothing to print</h1><p>Enter at least one suggestion.</p>
<p><a href="/vote">Back to the form</a></p></body></html>
"#;

pub const ERROR_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>Printing failed</title></head>
<body><h1>Printing failed</h1><p>The printer did not accept the job.</p>
<p><a href="/vote">Back to the form</a></p></body></html>
"#;

pub enum Outcome {
    Printed,
    Empty,
}

pub fn handle_vote(request: &mut Request, printer: &mut Printer<UsbDriver>) -> Result<Outcome> {
    let mut body = String::new();
    request
        .as_reader()
        .take(MAX_BODY)
        .read_to_string(&mut body)?;

    let mut suggestions = String::new();
    let mut name = String::new();
    for (key, value) in parse_form(&body) {
        match key.as_str() {
            "suggestions" => suggestions = value,
            "name" => name = value,
            _ => {}
        }
    }

    let name: String = name.trim().chars().take(MAX_NAME_CHARS).collect();
    let name = if name.is_empty() {
        "Anonymous"
    } else {
        name.as_str()
    };

    let suggestions: String = suggestions.chars().take(MAX_SUGGESTION_CHARS).collect();
    let lines: Vec<&str> = suggestions
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if lines.is_empty() {
        return Ok(Outcome::Empty);
    }

    // The printer flips each line individually, so the block is emitted back to
    // front to read correctly once the strip is turned around.
    let mut p = printer.init()?.upside_down(true)?;
    for line in lines.iter().rev() {
        p = p.writeln(line)?;
    }
    p.bold(true)?
        .writeln(&format!("Suggested by {name}"))?
        .bold(false)?
        .upside_down(false)?
        .feeds(8)?
        .print()?;

    Ok(Outcome::Printed)
}

fn parse_form(body: &str) -> Vec<(String, String)> {
    body.split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            (percent_decode(key), percent_decode(value))
        })
        .collect()
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                out.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                match (hex_value(bytes[index + 1]), hex_value(bytes[index + 2])) {
                    (Some(high), Some(low)) => {
                        out.push(high << 4 | low);
                        index += 3;
                    }
                    _ => {
                        out.push(b'%');
                        index += 1;
                    }
                }
            }
            byte => {
                out.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&out).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
