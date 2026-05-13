# 0.13.2 - 2026-05-12

* Fix sensors staying empty on inverters whose `ReadInput` packets don't
  match the upstream `ReadInputAll` byte layout (notably the EG4 18kPV).
  HA discovery now uses register shortname keys (`pv1_voltage`, `soc`,
  `battery_voltage`, etc.) backed by a per-datalog register cache.
* Curated ~25 sensors by default. New `mqtt.homeassistant.publish_all_registers`
  option exposes all 81 documented input registers as HA sensors.


# 0.13.1 - 2026-05-11

* Surface every option the binary requires/honors in the addon's
  Configuration tab: `read_only` (was missing and required, blocking
  startup), `strict_data_check`, `homeassistant_enabled`,
  `register_read_interval`, `inverter_timeout`, `verbose`,
  `human_timestamps`, `show_unknown`; per-inverter network and timing
  knobs; `mqtt.homeassistant` block; `mqtt.publish_individual_input`;
  scheduler block.
* Port the addon base image from Alpine to Debian Bookworm so it can
  consume the bookworm-based `mechmyday/eg4-bridge` image without
  COPYing into `/bin` (symlink-vs-directory conflict).
* Add per-arch `build.yaml` so the addon installer no longer warns about
  `BUILD_FROM`/`BUILD_VERSION`.
* Finish the `lxp-bridge` → `eg4-bridge` rename in `run.sh`.


# 0.13.0 - 2026-05-11 (first fork release)

* Renamed addon from `lxp-bridge` to `eg4-bridge`. Multi-arch images
  published to `mechmyday/eg4-bridge` on Docker Hub.


---

Earlier history (pre-fork) is preserved in
[celsworth/lxp-bridge](https://github.com/celsworth/lxp-bridge).
