Actix Tools
===========

[![crates.io](https://img.shields.io/crates/v/actix-tools.svg)](https://crates.io/crates/actix-tools)
[![dependency status](https://deps.rs/repo/github/loggi/actix-tools/status.svg)](https://deps.rs/repo/github/loggi/actix-tools)

This repository aggregates several tools used to include batteries to actix and actix-web applications.

Features
========

By default no actors are included, but only basic tooling (sentry, logging and 12-factor configuration).
You may include or disable them with the cargo features flag on your `Cargo.toml` file.

1. Configuration using ``Settings.toml`` and environmental variable overrides (``config``)
2. Sentry issue tracking for panic!s (``sentry``)
3. Json logs (``json_logs``)
4. Influxdb actor for metrics collection (``influx_actors``)
5. MQTT Pub/Sub Workers (``mqtt_actors``)
6. Redis Workers with connection pools (``redis_actors``)
7. Diesel Workers for postgres with connection pools  (``diesel_actors``)


Testing
=======

```
cargo test --all-features
```

Examples
========

Coming soon!
