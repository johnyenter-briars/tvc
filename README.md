# TVC 
---

- Simple light-weight 'TV Control' to allow developers to control a [CEC](https://www.cec-o-matic.com/)-compatiable TV over LAN HTTP
- Assumes [libcec](https://github.com/Pulse-Eight/libcec) has been compiled and installed on the machine
    - Compiled using the [instructions](https://github.com/Pulse-Eight/libcec/blob/master/docs/README.raspberrypi.md#compilation-using-the-linux-kernel-driver) for a Raspberry pi
- TVC has been tested against a [Raspberry Pi Zero](https://www.raspberrypi.com/products/raspberry-pi-zero/)
- TVC assumes it's running in the context of a 'cecsvc' service account - which has been given sudo permissions to the binary `cec-ctl`
    - See the systemd service file at [tvc.service](./setup/tvc.service)
