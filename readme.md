# Full page OS display

Motion sensor based display turn off means turning off the USB power on port level. Rust web server exposes a single endpoint to toggle, turn on/off the external screen. The endpoint itself is triggered via the HomeAssistant automation.

The USB power management is depending on the [uhubctl](https://github.com/mvp/uhubctl) project.

## Setup the Linux service

https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050

## Cross compiling for ARM

Good article for [Mac OS setup](https://sigmaris.info/blog/2019/02/cross-compiling-rust-on-mac-os-for-an-arm-linux-router/)
