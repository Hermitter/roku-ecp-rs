# roku-ecp-rs
> Expect breaking changes until 0.1.0 since Roku ECP has incorrect/missing documentation. 

[![Documentation](https://docs.rs/roku-ecp/badge.svg)](https://docs.rs/roku-ecp/)
[![Crate](https://img.shields.io/crates/v/roku-ecp.svg)](https://crates.io/crates/roku-ecp)

A Rust crate for Roku's [External Control Protocol](https://developer.roku.com/en-gb/docs/developer-program/debugging/external-control-api.md).

# Installation
Add the following to your Cargo.toml:
```
[dependencies]
roku_ecp = "0.0.2"
```

For usage, view the [examples](./examples).

# Available APIs

## General:
- [x] query/media-player
- [x] keydown/\<KEY>
- [x] keyup/\<KEY>
- [x] keypress/\<KEY>
- [x] launch/\<APP_ID>
- [x] install/\<APP_ID>
- [X] query/device-info
- [x] query/icon/\<APP_ID>
- [x] query/active-app/\<APP_ID>
- [x] query/apps/\<APP_ID>
- [x] search
- [ ] input (Accelerometer, Orientation, Gyroscope, Magnetometer, Touch and multi-touch)

## TV:
> Feel free to make a PR for these features. I do not have a Roku TV to test.
- [ ] query/tv-channels
- [ ] query/tv-active-channel
- [ ] launch/tvinput.dtv

# Dependencies

**Operating Systems**

Windows and macOS: None

Linux: OpenSSL + headers > v1.0.1
- Debian: `sudo apt install libssl-dev`
- Fedora: `sudo dnf install openssl-devel`

**Async Runtime**

This crate requires an asynchronous runtime such as
[tokio](https://github.com/tokio-rs/tokio) or
[async-std](https://github.com/async-rs/async-std).

# Finding Your Roku Device's IP
In the main menu, navigate to `Settings > Networking > About`.

# Credits

[@carloabelli's roku crate](https://github.com/carloabelli/roku) which was used as a reference.