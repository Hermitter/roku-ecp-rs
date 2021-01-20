# roku-ecp-rs
> Under Development

A Rust crate for the Roku External Control Protocol (ECP)

# API Roadmap
List of [APIs](https://developer.roku.com/en-gb/docs/developer-program/debugging/external-control-api.md#external-control-service-commands) to implement.
## General:

- [x] query/media-player
- [x] keydown/\<KEY>
- [x] keyup/\<KEY>
- [x] keypress/\<KEY>
- [ ] launch/\<APP_ID>
- [ ] install/\<APP_ID>
- [X] query/device-info
- [x] query/icon/\<APP_ID>
- [x] query/active-app/\<APP_ID>
- [x] query/apps/\<APP_ID>
- [ ] input
- [ ] search

## TV:
- [ ] query/tv-channels
- [ ] query/tv-active-channel
- [ ] launch/tvinput.dtv

# Credits

[@carloabelli's roku crate](https://github.com/carloabelli/roku) which was used as a reference.