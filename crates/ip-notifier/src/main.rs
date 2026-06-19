use eyre::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    io,
    net::IpAddr,
};
use time::{OffsetDateTime, macros::format_description};

const STATE_FILE: &str = "/run/ip-notifier.state";

fn main() -> Result<()> {
    let interfaces = collect_interfaces()?;
    let hashed_interfaces = hash_interfaces(&interfaces);
    if already_printed(&hashed_interfaces)? {
        return Ok(());
    }

    let hostname = hostname::get()?;
    let now = OffsetDateTime::now_local()?.format(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ))?;
    let text = format_interfaces(&hostname.to_string_lossy(), &now, &interfaces)?;

    print_client::print(&text)?;
    fs::write(STATE_FILE, &hashed_interfaces)?;

    Ok(())
}

fn hash_interfaces(interfaces: &BTreeMap<String, BTreeSet<IpAddr>>) -> String {
    let mut hasher = DefaultHasher::new();
    interfaces.hash(&mut hasher);
    hasher.finish().to_string()
}

fn already_printed(current_hash: &str) -> Result<bool> {
    match fs::read_to_string(STATE_FILE) {
        Ok(previous) => Ok(previous == current_hash),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}

fn collect_interfaces() -> Result<BTreeMap<String, BTreeSet<IpAddr>>> {
    let interfaces = if_addrs::get_if_addrs()?
        .into_iter()
        .filter(|iface| !iface.is_loopback())
        .fold(
            BTreeMap::<String, BTreeSet<IpAddr>>::new(),
            |mut interfaces, iface| {
                let ip = iface.ip();
                interfaces.entry(iface.name).or_default().insert(ip);
                interfaces
            },
        );

    Ok(interfaces)
}

fn format_interfaces(
    hostname: &str,
    now: &str,
    interfaces: &BTreeMap<String, BTreeSet<IpAddr>>,
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
