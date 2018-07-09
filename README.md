Actix Tools
===========

This repository aggregates several tools used to include batteries to actix and actix-web applications.

Features
========

By default no actors are included, but only basic tooling (Sentry and Logging). You may include them
with the cargo features flag on your `Cargo.toml` file.

1. Sentry issue tracking for panic!s (``sentry``)
2. Json logs (``json_logs``)
3. Influxdb actor for metrics collection (``influx_actors``)
4. MQTT Pub/Sub Workers (``mqtt_actors``)
5. Redis Workers with connection pools (``redis_actors``)
6. Diesel Workers for postgres with connection pools  (``diesel_actors``)


Testing
=======

```
cargo test --all-features
```
