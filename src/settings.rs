use config::{Config, Environment, File};
use serde::de::DeserializeOwned;

/// Gets app settings.
/// Will parse ``Settings.toml`` which may be overriden by environmental
/// variables starting with ``{crate_name}_``. One example is
/// ``MY_APP_LOG_LEVEL=debug``.
pub fn get_settings<T: DeserializeOwned>(prefix: String) -> T {
    let mut settings = Config::default();

    settings
        .merge(File::with_name("Settings").required(false))
        .expect("Failed to parse settings file")
        .merge(Environment::with_prefix(&prefix))
        .expect("Failed to parse settings env");

    settings.try_into().expect("Invalid settings")
}
