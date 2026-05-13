# eg4-bridge
[![Release](https://github.com/mechmyday/eg4-bridge/actions/workflows/release.yaml/badge.svg)](https://github.com/mechmyday/eg4-bridge/actions/workflows/release.yaml)

eg4-bridge is a Rust tool that talks to EG4 (and Luxpower-protocol-compatible) inverters on the local network and bridges their data to MQTT, InfluxDB, and SQL databases. It's a fork of [jaredmauch/eg4-bridge](https://github.com/jaredmauch/eg4-bridge), which forked from [celsworth/lxp-bridge](https://github.com/celsworth/lxp-bridge).

## What you get

- Local-only monitoring — no dependency on the manufacturer's cloud servers.
- MQTT publishing of every input/hold register, both raw and decoded.
- Home Assistant MQTT discovery for ~25 curated sensors out of the box (PV, battery, grid, EPS, temperatures, daily/lifetime energies). Opt-in to expose all 81 documented input registers.
- Optional InfluxDB v1 sink for time-series storage.
- Optional PostgreSQL / MySQL / SQLite sinks; schema migrations run automatically.

## Home Assistant addon

The HA addon is supported and tested on the EG4 18kPV. To install:

1. In Home Assistant: **Settings → Add-ons → Add-on Store → ⋮ → Repositories**, add `https://github.com/mechmyday/eg4-bridge`.
2. Install the **eg4-bridge** addon (or **eg4-bridge (dev)** to track `main`).
3. Open the addon's **Configuration** tab, fill in the inverter `host`, `serial`, `datalog`, and your MQTT broker details. Set `homeassistant_enabled: true` and `mqtt.homeassistant.enabled: true` to publish discovery messages.
4. Start the addon. New entities should appear under the eg4-bridge device within one register-read cycle (default 60s).

The `inputs/all` MQTT payload uses register shortnames (`pv1_voltage`, `battery_voltage`, `soc`, …) sourced from `doc/eg4_registers.json`. Values are published raw; HA's `value_template` applies the per-register scale factor. Set `mqtt.homeassistant.publish_all_registers: true` to expose all 81 input registers as HA sensors instead of the curated essentials.

## Standalone (non-HA) usage

Build from source with Rust 1.88+ (`cargo install --path .`) or pull the multi-arch image from Docker Hub:

```
docker run --rm -v "$PWD/config.yaml:/etc/config.yaml" mechmyday/eg4-bridge:v0.13.2
```

See `config.yaml.example` for a fully-commented configuration covering MQTT, InfluxDB, databases, the scheduler, and per-inverter knobs.

### Database backends

`databases:` entries accept PostgreSQL, MySQL, or SQLite URLs. PostgreSQL is recommended for production:

```yaml
databases:
- enabled: true
  url: postgres://user@localhost/eg4_bridge
```

Trust auth (no password) and TCP-to-localhost are well-tested. Unix socket connections (`postgres://user@/db?host=/var/run/postgresql`) need URL-validation improvements — use TCP for now.

## Documentation

- Register definitions and metadata live in [`doc/eg4_registers.json`](doc/eg4_registers.json) — this is the source of truth for shortnames, units, and scales.
- Modbus reference: [`doc/EG4-18KPV-12LV-Modbus-Protocol.pdf`](doc/EG4-18KPV-12LV-Modbus-Protocol.pdf).
- Captured wire traces and historical notes are in `doc/*.txt`.

## Contributing

Issues and pull requests welcome. When reporting bugs, please include inverter model, firmware version, datalog/inverter serial prefix, and a snippet of `loglevel: debug` output around the misbehavior. Co-maintainers will be considered for sustained contributors.
