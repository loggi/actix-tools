use config::{Config, Environment, File};
use serde::de::DeserializeOwned;

/// Gets app settings.
/// Will parse ``Settings.toml`` which may be overriden by environmental
/// variables starting with ``{crate_name}_``. One example is
/// ``MY_APP_LOG_LEVEL=debug``.
pub fn get_settings<T: DeserializeOwned>() -> T {
    let mut settings = Config::default();
    let crate_name = env!("CARGO_PKG_NAME").replace("-", "_");

    settings
        .merge(File::with_name("Settings").required(false))
        .expect("Failed to parse settings file")
        .merge(Environment::with_prefix(&crate_name))
        .expect("Failed to parse settings env");

    settings.try_into().expect("Invalid settings")
}
