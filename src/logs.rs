use chrono::Utc;
use env_logger;
use env_logger::Formatter;
use log::Record;
use serde_json;
use std::io::{Error, Write};

#[derive(Clone, Deserialize)]
pub struct LogSettings {
    pub level:     String,
    pub backtrace: String,
}

pub fn formatter(buffer: &mut Formatter, record: &Record) -> Result<(), Error> {
    let data = json!({
        "level": record.level().to_string(),
        "message": record.args(),
        "timestamp": Utc::now(),
    });
    let payload = serde_json::to_string(&data)?;
    writeln!(buffer, "{}", payload)
}

pub fn init_logger(settings: LogSettings) {
    ::std::env::set_var("RUST_BACKTRACE", settings.backtrace);
    ::std::env::set_var("RUST_LOG", settings.level);

    env_logger::Builder::from_default_env()
        .format(formatter)
        .default_format_timestamp(false)
        .init();
}
