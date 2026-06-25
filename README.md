# funny-feed

An Epson TM-U220 side project.

## Setup

### Printer access

To allow logged-in users besides root to access the printer, install the
appropriate udev rule.

If you're accessing the device via SSH:

```bash
sudo install -m 644 setup/50-epson-printer-group.rules \
    /etc/udev/rules.d/50-epson-printer-group.rules
```

If you have a local session:

```bash
sudo install -m 644 setup/50-epson-printer-tag.rules \
    /etc/udev/rules.d/50-epson-printer-tag.rules
```

### Printing on network changes

mDNS is an elegant way to get the IP of the device once it's in the network,
but it can be hard to use.

To reliably print the network interfaces whenever NetworkManager reports a
change, install the `ip-notifier` binary and the dispatcher script:

```bash
cargo build --release -p ip-notifier
sudo install -m 755 target/release/ip-notifier /usr/local/bin/ip-notifier
sudo install -o root -g root -m 755 setup/50-ip-notifier \
    /etc/NetworkManager/dispatcher.d/50-ip-notifier
```

### Running the print server

We're also using udev rules to restart the print server's systemd unit whenever
the printer is plugged in, which starts it at boot and gives it a fresh USB
connection on every replug.

```bash
cargo build --release -p print-server
sudo install -m 755 target/release/print-server /usr/local/bin/print-server
sudo install -m 644 setup/print-server.service /etc/systemd/system/print-server.service
sudo install -m 644 setup/50-print-server.rules /etc/udev/rules.d/50-print-server.rules
```

To make it available from the outside, add this block to a server directive
in your nginx config.

```nginx
location = /print {
    proxy_set_header X-Real-IP $remote_addr;
    proxy_pass http://127.0.0.1:8080;
}
```

## Print server usage

The print server listens on `127.0.0.1:8080`. Send the text to print as the
body of a `POST /print` request. Each line is printed and the paper is cut
afterwards:

```bash
curl --data-binary 'Hello from curl' http://localhost:8080/print
```

Multi-line input works too:

```bash
printf 'Line one\nLine two' | curl --data-binary @- http://localhost:8080/print
```

### Lunch menu

A `GET /food` request fetches the daily menu feed and prints each restaurant
with its dishes and prices, finished off with the mascot:

```bash
curl http://localhost:8080/food
```

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
