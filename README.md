# funny-feed

An Epson TM-U220 side project.

## Setup

To allow logged-in users besides root to access the printer, you need to copy
the appropriate udev rule to `/etc/udev/rules.d`:

- If you're accessing the device via SSH, use `setup/50-epson-printer-group.rules`
- If you have a local session, use `setup/50-epson-printer-tag.rules`

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
