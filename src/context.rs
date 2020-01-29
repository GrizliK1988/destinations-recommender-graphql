use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub(crate) struct Context {
    pub(crate) pool: Pool<ConnectionManager<PgConnection>>,
}

pub(crate) fn create_context() -> Context {
    dotenv().ok();

    let connection_string = env::var("DATABASE_URL")
        .expect("DATABASE_URL env variable should be defined");

    let connection_manager = r2d2_diesel::ConnectionManager::new(connection_string);
    let pool = Pool::<ConnectionManager<PgConnection>>::builder().max_size(10).build(connection_manager).unwrap();

    Context { pool }
}
