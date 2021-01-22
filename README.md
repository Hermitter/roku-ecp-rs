# roku-ecp-rs
> Under Development

[![Documentation](https://docs.rs/roku-ecp/badge.svg)](https://docs.rs/roku-ecp/)
[![Crate](https://img.shields.io/crates/v/roku-ecp.svg)](https://crates.io/crates/roku-ecp)

A Rust crate for Roku's [External Control Protocol](https://developer.roku.com/en-gb/docs/developer-program/debugging/external-control-api.md).

# ToDos
- [ ] Examples
- [ ] Documentation

# Available APIs
> Some APIs might be missing since I could not find concrete documentation.

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

# Credits

[@carloabelli's roku crate](https://github.com/carloabelli/roku) which was used as a reference.