# Home Assistant addon: eg4-bridge (dev)

> This is the **dev** version of eg4-bridge — its Docker image is rebuilt on every push to `main`.
> Only use this in preference to the stable `eg4-bridge` addon if you need an unreleased change.

Allows local communication with EG4 (and Luxpower-protocol-compatible) inverters and bridges their data to MQTT, including Home Assistant MQTT discovery.

![Supports aarch64 Architecture][aarch64-shield] ![Supports amd64 Architecture][amd64-shield] ![Supports armv7 Architecture][armv7-shield]

## About

eg4-bridge is a Rust tool that talks to EG4 inverters (commonly used with home-battery and solar setups) on the local network. It lets you monitor and control the inverter without depending on the manufacturer's cloud.

## Configuration

Configuration matches the stable addon — see the [project README](https://github.com/mechmyday/eg4-bridge#readme) for the configuration reference.

[aarch64-shield]: https://img.shields.io/badge/aarch64-yes-green.svg
[amd64-shield]: https://img.shields.io/badge/amd64-yes-green.svg
[armv7-shield]: https://img.shields.io/badge/armv7-yes-green.svg
