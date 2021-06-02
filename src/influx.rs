use actix::prelude::*;
use influx_db_client::{Client, Point, Value};

#[derive(Clone, Deserialize)]
pub struct InfluxSettings {
    pub uri:      String,
    pub db:       String,
    pub user:     String,
    pub password: String,
}

pub fn get_client(settings: InfluxSettings) -> Client {
    Client::new(settings.uri, settings.db)
        .set_authentication(settings.user, settings.password)
}

pub fn send(client: Client) {
    let point = Point::new("test1")
        .add_tag("tags", Value::String("filter".to_string()))
        .add_field("count", Value::Integer(1))
        .to_owned();

    // if Precision is None, the default is second
    // Multiple write
    let _ = client.write_point(point, None, None).unwrap();
}

pub struct InfluxEmitter {
    client: Client,
}

impl InfluxEmitter {
    pub fn from_settings(settings: InfluxSettings) -> Self {
        let client = get_client(settings);
        Self { client }
    }
}

#[derive(Message, Debug)]
pub struct Metric(pub Point);

impl Actor for InfluxEmitter {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Starting Influx Emitter");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Stopping Influx Emitter");
    }
}

impl Handler<Metric> for InfluxEmitter {
    type Result = ();

    fn handle(&mut self, msg: Metric, _ctx: &mut Self::Context) {
        let f = self.client.write_point(msg.0, None, None).unwrap();
    }
}
