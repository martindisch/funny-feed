# funny-feed

An Epson TM-U220 side project.

## Usage

To allow logged-in users besides root to access the printer, add this to
`/etc/udev/rules.d/50-epson-printer.rules`:

```
ACTION!="remove", SUBSYSTEMS=="usb", ATTRS{idVendor}=="04b8", ATTRS{idProduct}=="0202", MODE="0660", TAG+="uaccess"
```

If you're accessing the device via SSH, you need to do group assignment instead:

```
ACTION!="remove", SUBSYSTEMS=="usb", ATTRS{idVendor}=="04b8", ATTRS{idProduct}=="0202", MODE="0660", GROUP="plugdev"

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
