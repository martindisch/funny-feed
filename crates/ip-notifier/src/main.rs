use eyre::Result;
use std::{collections::BTreeMap, fmt::Write, net::IpAddr};
use time::{OffsetDateTime, macros::format_description};

fn main() -> Result<()> {
    let hostname = hostname::get()?;
    let now = OffsetDateTime::now_local()?.format(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ))?;
    let interfaces = collect_interfaces()?;
    let text = format_interfaces(&hostname.to_string_lossy(), &now, &interfaces)?;

    print_client::print(&text)?;

    Ok(())
}

fn collect_interfaces() -> Result<BTreeMap<String, Vec<IpAddr>>> {
    let interfaces = if_addrs::get_if_addrs()?
        .into_iter()
        .filter(|iface| !iface.is_loopback())
        .fold(
            BTreeMap::<String, Vec<IpAddr>>::new(),
            |mut interfaces, iface| {
                let ip = iface.ip();
                interfaces.entry(iface.name).or_default().push(ip);
                interfaces
            },
        );

    Ok(interfaces)
}

fn format_interfaces(
    hostname: &str,
    now: &str,
    interfaces: &BTreeMap<String, Vec<IpAddr>>,
) -> Result<String> {
    let title = format!("Current network interfaces of\n{hostname}\n{now}\n");
    if interfaces.is_empty() {
        return Ok(format!("{title}\nNo network interfaces found.\n"));
    }

    let mut text = title;
    for (name, ips) in interfaces {
        write!(text, "\n{name}")?;
        for ip in ips {
            write!(text, "\n  {ip}")?;
        }
        text.push('\n');
    }

    Ok(text)
}
