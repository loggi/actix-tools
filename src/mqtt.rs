use actix::prelude::*;
pub use rumqtt::{Message as MqttMessage, MqttCallback, MqttClient, MqttOptions, QoS};

#[derive(Clone, Deserialize)]
pub struct MqttSettings {
    pub broker:        String,
    pub keep_alive:    usize,
    pub reconnect:     usize,
    pub clean_session: bool,
    pub client_id:     String,
}

impl Into<MqttOptions> for MqttSettings {
    fn into(self) -> MqttOptions {
        MqttOptions::new()
            .set_reconnect(self.reconnect as u16)
            .set_keep_alive(self.keep_alive as u16)
            .set_clean_session(self.clean_session)
            .set_client_id(self.client_id.as_str())
            .set_broker(&self.broker)
    }
}

pub trait MqttListener: Actor
where
    Self::Context: AsyncContext<Self>, {
    /// Get MqttListener settings.
    fn settings(&self) -> MqttSettings;

    fn subscriptions(&self) -> Vec<String>;

    fn callback(addr: &Addr<Self>, msg: MqttMessage);

    fn listen(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let callback = MqttCallback::new().on_message(move |msg| {
            Self::callback(&addr, msg);
        });

        let mut client =
            MqttClient::start(self.settings().into(), Some(callback)).unwrap();

        client
            .subscribe(
                self.subscriptions()
                    .iter()
                    .map(|topic| (topic.as_str(), QoS::Level0))
                    .collect(),
            )
            .unwrap();
    }
}
