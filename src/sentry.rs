pub use sentry_client;

#[derive(Clone, Deserialize)]
pub struct SentrySettings {
    pub dsn: String,
}

pub fn init_sentry(settings: SentrySettings) {
    sentry_client::init((
        settings.dsn,
        sentry_client::ClientOptions {
            release: sentry_crate_release!(),
            ..Default::default()
        },
    ));
    sentry_client::integrations::panic::register_panic_handler();
    info!("Starting Sentry integration")
}
