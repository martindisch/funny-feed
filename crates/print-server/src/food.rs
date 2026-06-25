use escpos::{driver::UsbDriver, printer::Printer, utils::JustifyMode};
use eyre::{Result, eyre};
use serde::Deserialize;

const FOOD_URL: &str = "https://eat.devinite.dev/data.json";
const WIDTH: usize = 40;

const MASCOT: &str = r#"               __
   .,-;-;-,.../'_\
  _/_/_/_|_\_\)  /
'-<_><_><_><_>=/\
  `|_|===/_/--`\_\
   ""     ""    ""
"#;

#[derive(Debug, Deserialize)]
struct RawData {
    date: String,
    results: Vec<MenuResult>,
}

#[derive(Debug, Deserialize)]
struct MenuResult {
    restaurant: Restaurant,
    status: String,
    #[serde(default)]
    items: Vec<MenuItem>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Restaurant {
    name: String,
}

#[derive(Debug, Deserialize)]
struct MenuItem {
    name: String,
    #[serde(default)]
    price: Option<String>,
}

pub fn handle_food(printer: &mut Printer<UsbDriver>) -> Result<()> {
    let data = fetch()?;
    println!(
        "food feed for {} with {} restaurants",
        data.date,
        data.results.len()
    );

    let mut p = printer.init()?;
    p = p
        .justify(JustifyMode::CENTER)?
        .bold(true)?
        .writeln(&format!("Menu {}", data.date))?
        .bold(false)?
        .justify(JustifyMode::LEFT)?
        .feed()?;

    for result in &data.results {
        p = p.bold(true)?;
        for line in wrap(&result.restaurant.name, WIDTH) {
            p = p.writeln(&line)?;
        }
        p = p.bold(false)?;

        match result.status.as_str() {
            "ok" if !result.items.is_empty() => {
                for item in &result.items {
                    for line in format_item(&item.name, item.price.as_deref()) {
                        p = p.writeln(&line)?;
                    }
                }
            }
            "ok" | "no-menu" => {
                p = p.writeln("  No menu today")?;
            }
            "error" => {
                p = p.writeln("  Menu unavailable")?;
                if let Some(error) = &result.error {
                    for line in wrap(error, WIDTH) {
                        p = p.writeln(&line)?;
                    }
                }
            }
            other => {
                p = p.writeln(&format!("  ({other})"))?;
            }
        }

        p = p.feed()?;
    }

    p = p
        .justify(JustifyMode::CENTER)?
        .feed()?
        .writeln("Food facts by")?
        .justify(JustifyMode::LEFT)?;
    for line in center_block(MASCOT) {
        p = p.writeln(&line)?;
    }
    p = p
        .justify(JustifyMode::CENTER)?
        .writeln("eat.devinite.dev")?
        .justify(JustifyMode::LEFT)?;

    p.print_cut()?;

    Ok(())
}

fn fetch() -> Result<RawData> {
    let response = minreq::get(FOOD_URL).send()?;
    match response.status_code {
        200..=299 => Ok(serde_json::from_str(response.as_str()?)?),
        status => Err(eyre!(
            "food feed returned {}: {}",
            status,
            response.as_str().unwrap_or("<non-text body>").trim()
        )),
    }
}

fn format_item(name: &str, price: Option<&str>) -> Vec<String> {
    let mut lines = wrap(name, WIDTH);
    if lines.is_empty() {
        lines.push(String::new());
    }

    let Some(price) = price else {
        return lines;
    };
    let price_len = price.chars().count();

    let last_idx = lines.len() - 1;
    let last_len = lines[last_idx].chars().count();
    if last_len + price_len + 2 <= WIDTH {
        let dots = WIDTH - last_len - price_len;
        lines[last_idx] = format!("{}{} {price}", lines[last_idx], ".".repeat(dots - 1));
    } else {
        let dots = WIDTH.saturating_sub(price_len + 1);
        lines.push(format!("{} {price}", ".".repeat(dots)));
    }

    lines
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let current_len = current.chars().count();
        if current.is_empty() {
            current.push_str(word);
        } else if current_len + 1 + word.chars().count() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn center_block(block: &str) -> Vec<String> {
    let max = block.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let prefix = " ".repeat(WIDTH.saturating_sub(max) / 2);
    block.lines().map(|l| format!("{prefix}{l}")).collect()
}
