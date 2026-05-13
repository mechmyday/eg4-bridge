# 0.13.2 - 2026-05-12

* Fix Home Assistant sensors staying empty on inverters whose `ReadInput`
  packets don't fit the upstream `ReadInputAll` byte layout (e.g. EG4 18kPV
  sends data in 254-byte chunks at registers 0/127/254). The `inputs/all`
  MQTT topic is now sourced from a per-datalog register cache and uses
  register shortnames (`pv1_voltage`, `battery_voltage`, `soc`, etc.)
  derived from `doc/eg4_registers.json`. Raw u16 values are published; HA's
  `value_template` applies the per-register scale factor.
* HA discovery is now metadata-driven. ~25 curated sensors by default; the
  new `mqtt.homeassistant.publish_all_registers` option exposes all 81
  documented input registers.
* `ReadInputAll → inputs/all` MQTT publish removed (cache path is now the
  single writer for that topic). InfluxDB and SQL pipelines unchanged.
* Ship `doc/eg4_registers.json` into the build image so the new
  `include_str!` resolves.


# 0.13.1 - 2026-05-11

* Close addon-vs-binary configuration gaps. The addon's generated
  `/etc/config.yaml` now exposes every option the binary requires/honors:
  the previously-missing `read_only`, `strict_data_check`,
  `homeassistant_enabled`, `register_read_interval`, `inverter_timeout`,
  `verbose`, `human_timestamps`, `show_unknown`; per-inverter
  `use_tcp_nodelay`, `read_timeout`, `register_block_size`, `delay_ms`,
  `read_only`, `register_read_interval`; the `mqtt.homeassistant`
  sub-block; `mqtt.publish_individual_input`; and the `scheduler` block.
* Sync `config.yaml.example` to the same set; fix the silently-ignored
  `topic:` example key (the binary parses `namespace:`).
* Port the HA addon base image from Alpine to Debian Bookworm to match the
  bookworm runtime and resolve the `/bin` directory-vs-symlink COPY
  conflict.
* Add per-arch `build.yaml` files so the addon installer no longer warns
  about missing `BUILD_FROM`/`BUILD_VERSION`.
* Finish renaming references from `lxp-bridge` to `eg4-bridge` in
  `addon/run.sh` and `addon.dev/run.sh`.


# 0.13.0 - 2026-05-11 (first fork release)

* Switch TLS stack from native-tls/OpenSSL to rustls (sqlx
  `runtime-tokio-rustls`, reqwest `rustls` feature). Fixes the
  arm-cross-compile link error caused by the proc-macro host build
  picking up ARM OpenSSL libs.
* Modernize the CI Dockerfile: rust 1.88, Debian bookworm runtime, drop
  the OpenSSL cross-build step.
* Modernize GitHub Actions workflows: bump action pins, replace archived
  `actions-rs/*` with `dtolnay/rust-toolchain@stable`, trigger on `main`
  with `workflow_dispatch`. Drop the unmaintained darwin-amd64 build.
* Rename project, addon, binary, and Docker image references from
  `lxp-bridge`/`jaredmauch/eg4-bridge` to `mechmyday/eg4-bridge`.
* Re-publish multi-arch Docker images to `mechmyday/eg4-bridge` on
  Docker Hub.


---

Earlier history (pre-fork) is preserved in
[celsworth/lxp-bridge](https://github.com/celsworth/lxp-bridge) and
[jaredmauch/eg4-bridge](https://github.com/jaredmauch/eg4-bridge).
