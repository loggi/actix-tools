use actix::prelude::*;
use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;
use utils::n_workers;

pub use redis_client::*;

#[derive(Clone, Deserialize)]
pub struct RedisSettings {
    pub host:         String,
    pub port:         Option<u16>,
    pub password:     Option<String>,
    pub db:           Option<u16>,
    pub block_factor: f32,
}

impl RedisSettings {
    pub fn connection_info(&self) -> ConnectionInfo {
        let addr = ConnectionAddr::Tcp(self.host.clone(), self.port.unwrap_or(6379));

        ConnectionInfo {
            addr:   Box::new(addr),
            passwd: self.password.clone(),
            db:     self.db.unwrap_or(0) as i64,
        }
    }
}

/// Redis connection pool.
pub type RedisPool = Pool<RedisConnectionManager>;

/// Redis connection wrapper.
pub type RedisConnectionWrapper = PooledConnection<RedisConnectionManager>;

/// Redis actor worker.
pub struct RedisWorker {
    pub pool: RedisPool,
}

impl RedisWorker {
    pub fn from_pool(pool: RedisPool) -> Self {
        Self { pool }
    }

    pub fn get_connection(&self) -> RedisConnectionWrapper {
        self.pool.get().unwrap()
    }

    pub fn execute<F, R>(&self, exec: F) -> R
    where
        F: Fn(&Connection) -> R, {
        let connection = self.get_connection();
        exec(&connection)
    }
}

impl Actor for RedisWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Starting RedisWorker");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Stopping RedisWorker");
    }
}

/// Creates a new Redis connection pool.
pub fn create_pool(settings: RedisSettings) -> RedisPool {
    let manager = RedisConnectionManager::new(settings.clone().connection_info())
        .expect("Failed to connect to redis");

    Pool::builder()
        .max_size(n_workers(settings.block_factor) as u32)
        .build(manager)
        .expect("Failed to start redis connection pool")
}

pub fn create_workers(settings: RedisSettings) -> Addr<RedisWorker> {
    let redis_pool = create_pool(settings.clone());
    let workers = n_workers(settings.block_factor);

    SyncArbiter::start(workers, move || RedisWorker::from_pool(redis_pool.clone()))
}
