pub use sentry_client;

#[derive(Clone, Deserialize)]
pub struct SentrySettings {
    pub dsn: String,
}

pub fn init_sentry(settings: SentrySettings) -> sentry_client::ClientInitGuard {
    info!("Starting Sentry integration");
    sentry_client::init((
        settings.dsn,
        sentry_client::ClientOptions {
            release: sentry_client::release_name!(),
            ..Default::default()
        })
    )
}
