use actix::prelude::*;
use diesel::r2d2::ConnectionManager as DieselConnectionManager;
use r2d2::{Pool, PooledConnection};
use utils::n_workers;

pub use diesel_core::*;

#[derive(Clone, Deserialize)]
pub struct DieselSettings {
    pub host:         String,
    pub port:         Option<u16>,
    pub user:         Option<String>,
    pub password:     Option<String>,
    pub db:           Option<String>,
    pub block_factor: f32,
}

impl DieselSettings {
    pub fn connection_info(&self) -> String {
        let auth = self
            .password
            .clone()
            .map(|password| {
                format!("{}:{}@", self.user.clone().unwrap_or_default(), password)
            })
            .unwrap_or_default();

        let port = self.port.unwrap_or(5432);
        let db = self
            .db
            .clone()
            .map(|db| format!("/{}", db))
            .unwrap_or_default();

        format!("postgres://{}{}:{}{}", auth, self.host, port, db)
    }
}

/// Redis connection pool.
pub type DieselPool = Pool<DieselConnectionManager<PgConnection>>;

/// Redis connection wrapper.
pub type DieselConnectionWrapper =
    PooledConnection<DieselConnectionManager<PgConnection>>;

/// Redis actor worker.
pub struct DieselWorker {
    pub pool: DieselPool,
}

impl DieselWorker {
    pub fn from_pool(pool: DieselPool) -> Self {
        Self { pool }
    }

    pub fn get_connection(&self) -> DieselConnectionWrapper {
        self.pool.get().unwrap()
    }

    pub fn execute<F, R>(&self, exec: F) -> R
    where
        F: Fn(&PgConnection) -> R, {
        let connection = self.get_connection();
        exec(&connection)
    }
}

impl Actor for DieselWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Starting DieselWorker");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Stopping DieselWorker");
    }
}

/// Creates a new Redis connection pool.
pub fn create_pool(settings: DieselSettings) -> DieselPool {
    let manager = DieselConnectionManager::new(settings.clone().connection_info());

    Pool::builder()
        .max_size(n_workers(settings.block_factor) as u32)
        .build(manager)
        .expect("Failed to start redis connection pool")
}

pub fn create_workers(settings: DieselSettings) -> Addr<DieselWorker> {
    let diesel_pool = create_pool(settings.clone());
    let workers = n_workers(settings.block_factor);

    SyncArbiter::start(workers, move || {
        DieselWorker::from_pool(diesel_pool.clone())
    })
}
