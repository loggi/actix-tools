extern crate actix;
#[cfg(feature = "diesel")]
extern crate diesel as diesel_core;
#[cfg(feature = "env_logger")]
extern crate env_logger;
#[cfg(feature = "influx_db_client")]
extern crate influx_db_client;
extern crate num_cpus;
#[macro_use]
extern crate log;
#[cfg(feature = "r2d2")]
extern crate r2d2;
#[cfg(feature = "r2d2_redis")]
extern crate r2d2_redis;
#[cfg(feature = "redis")]
extern crate redis as redis_client;
#[cfg(feature = "rumqtt")]
extern crate rumqtt;
#[cfg(feature = "sentry")]
#[macro_use]
extern crate sentry as sentry_client;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(feature = "diesel_actors")]
pub mod diesel;

#[cfg(feature = "influx_actors")]
pub mod influx;

#[cfg(feature = "json_logs")]
pub mod logs;

#[cfg(feature = "mqtt_actors")]
pub mod mqtt;

#[cfg(feature = "redis_actors")]
pub mod redis;

#[cfg(feature = "sentry")]
pub mod sentry;

pub mod utils;
