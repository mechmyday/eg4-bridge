# Home Assistant addon: eg4-bridge

Allows local communication with EG4 (and Luxpower-protocol-compatible) inverters and bridges their data to MQTT, including Home Assistant MQTT discovery.

![Supports aarch64 Architecture][aarch64-shield] ![Supports amd64 Architecture][amd64-shield] ![Supports armv7 Architecture][armv7-shield]

## About

eg4-bridge is a Rust tool that talks to EG4 inverters (commonly used with home-battery and solar setups) on the local network. It lets you monitor and control the inverter without depending on the manufacturer's cloud.

## Configuration

After installing the addon, open its **Configuration** tab and at minimum fill in:

- `inverters[0].host` — the inverter's local IP or hostname
- `inverters[0].serial` — 10-character inverter serial number
- `inverters[0].datalog` — 10-character datalog/wifi-dongle serial number
- `mqtt.host` / `mqtt.username` / `mqtt.password` — your MQTT broker
- `homeassistant_enabled: true` and `mqtt.homeassistant.enabled: true` — to publish HA discovery messages

By default the addon publishes a curated set of ~25 HA sensors (PV, battery, grid, EPS, temperatures, daily/lifetime energies). Set `mqtt.homeassistant.publish_all_registers: true` to expose all 81 documented input registers as HA sensors instead.

See the [project README](https://github.com/mechmyday/eg4-bridge#readme) for the configuration reference and source.

[aarch64-shield]: https://img.shields.io/badge/aarch64-yes-green.svg
[amd64-shield]: https://img.shields.io/badge/amd64-yes-green.svg
[armv7-shield]: https://img.shields.io/badge/armv7-yes-green.svg
