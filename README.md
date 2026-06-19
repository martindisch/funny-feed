# funny-feed

An Epson TM-U220 side project.

## Setup

### Printer access

To allow logged-in users besides root to access the printer, you need to copy
the appropriate udev rule to `/etc/udev/rules.d`:

- If you're accessing the device via SSH, use `setup/50-epson-printer-group.rules`
- If you have a local session, use `setup/50-epson-printer-tag.rules`

### Printing on network changes

mDNS is an elegant way to get the IP of the device once it's in the network,
but it can be hard to use.

To reliably print the network interfaces whenever NetworkManager reports a
change, install the `ip-notifier` binary and the dispatcher script:

```bash
cargo install --path crates/ip-notifier --root /usr/local
sudo install -o root -g root -m 755 setup/50-ip-notifier \
    /etc/NetworkManager/dispatcher.d/50-ip-notifier
```

## Print server usage

The print server listens on `0.0.0.0:8080`. Send the text to print as the body
of a `POST /print` request. Each line is printed and the paper is cut
afterwards:

```bash
curl --data-binary 'Hello from curl' http://localhost:8080/print
```

Multi-line input works too:

```bash
echo 'Line one\nLine two' | curl --data-binary @- http://localhost:8080/print
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
